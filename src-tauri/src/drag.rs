// Copyright 2023-2023 CrabNebula Ltd.
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::{collections::HashMap, path::PathBuf, sync::mpsc::channel};

use serde::{ser::Serializer, Deserialize, Deserializer, Serialize};
use tauri::{
    ipc::CallbackFn,
    AppHandle, Runtime, WebviewWindow,
};

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Drag(#[from] drag::Error),
    #[error(transparent)]
    Tauri(#[from] tauri::Error),
    #[error(transparent)]
    Base64(#[from] base64::DecodeError),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub struct Base64Image(String);

impl<'de> Deserialize<'de> for Base64Image {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        if let Some(data) = value.strip_prefix("data:image/png;base64,") {
            return Ok(Self(data.into()));
        }
        Err(serde::de::Error::custom(
            "expected an image/png base64 image string",
        ))
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Image {
    Base64(Base64Image),
    Raw(drag::Image),
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum DragItem {
    // A list of files to be dragged.
    //
    // The paths must be absolute.
    Files(Vec<PathBuf>),
    // Data to share with another app.
    Data {
        data: SharedData,
        types: Vec<String>,
    },
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum SharedData {
    Fixed(String),
    Map(HashMap<String, String>),
}

#[derive(Serialize)]
struct CallbackResult {
    result: drag::DragResult,
    #[serde(rename = "cursorPos")]
    cursor_pos: drag::CursorPosition,
}

#[tauri::command]
pub async fn start_drag<R: Runtime>(
    app: AppHandle<R>,
    window: WebviewWindow<R>,
    item: DragItem,
    image: Image,
    on_event_fn: Option<CallbackFn>,
) -> Result<()> {
    let (tx, rx) = channel();

    let image = match image {
        Image::Raw(r) => r,
        Image::Base64(b) => {
            use base64::Engine;
            drag::Image::Raw(base64::engine::general_purpose::STANDARD.decode(b.0)?)
        }
    };

    app.run_on_main_thread(move || {
        let raw_window = tauri::Result::Ok(window.clone());

        let r = match raw_window {
            Ok(w) => drag::start_drag(
                &w,
                match item {
                    DragItem::Files(f) => drag::DragItem::Files(f),
                    DragItem::Data { data, types } => drag::DragItem::Data {
                        provider: Box::new(move |data_type| match &data {
                            SharedData::Fixed(d) => Some(d.as_bytes().to_vec()),
                            SharedData::Map(m) => m.get(data_type).map(|d| d.as_bytes().to_vec()),
                        }),
                        types,
                    },
                },
                image,
                move |result, cursor_pos| {
                    if let Some(on_event_fn) = on_event_fn {
                        let callback_result = CallbackResult { result, cursor_pos };
                        let js = format_callback(
                            on_event_fn,
                            &serde_json::to_string(&callback_result).unwrap(),
                        )
                        .expect("unable to serialize DragResult");

                        let _ = window.eval(js.as_str());
                    }
                },
                Default::default(),
            )
            .map_err(Into::into),
            Err(e) => Err(e.into()),
        };
        tx.send(r).unwrap();
    })?;

    rx.recv().unwrap()
}

// Initializes the plugin.
// pub fn init<R: Runtime>() -> TauriPlugin<R> {
//     Builder::new("drag")
//         .invoke_handler(tauri::generate_handler![start_drag])
//         .js_init_script(include_str!("./api-iife.js").to_string())
//         .build()
// }

use serde_json::value::RawValue;
use serialize_to_javascript::Serialized;

// The information about this is quite limited. On Chrome/Edge and Firefox, [the maximum string size is approximately 1 GB](https://stackoverflow.com/a/34958490).
//
// [From MDN:](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/length#description)
//
// ECMAScript 2016 (ed. 7) established a maximum length of 2^53 - 1 elements. Previously, no maximum length was specified.
//
// In Firefox, strings have a maximum length of 2\*\*30 - 2 (~1GB). In versions prior to Firefox 65, the maximum length was 2\*\*28 - 1 (~256MB).
const MAX_JSON_STR_LEN: usize = usize::pow(2, 30) - 2;

// Minimum size JSON needs to be in order to convert it to JSON.parse with [`format_json`].
// TODO: this number should be benchmarked and checked for optimal range, I set 10 KiB arbitrarily
// we don't want to lose the gained object parsing time to extra allocations preparing it
const MIN_JSON_PARSE_LEN: usize = 10_240;

// Transforms & escapes a JSON value.
//
// If it's an object or array, JSON.parse('{json}') is used, with the '{json}' string properly escaped.
// The return value of this function can be safely used on [`eval`](crate::Window#method.eval) calls.
//
// Single quotes chosen because double quotes are already used in JSON. With single quotes, we only
// need to escape strings that include backslashes or single quotes. If we used double quotes, then
// there would be no cases that a string doesn't need escaping.
//
// The function takes a closure to handle the escaped string in order to avoid unnecessary allocations.
//
// # Safety
//
// The ability to safely escape JSON into a JSON.parse('{json}') relies entirely on 2 things.
//
// 1. `serde_json`'s ability to correctly escape and format json into a string.
// 2. JavaScript engines not accepting anything except another unescaped, literal single quote
//     character to end a string that was opened with it.
fn serialize_js_with<T: Serialize, F: FnOnce(&str) -> String>(
  value: &T,
  options: serialize_to_javascript::Options,
  cb: F,
) -> Result<String> {
  // get a raw &str representation of a serialized json value.
  let string = serde_json::to_string(value).unwrap();
  let raw = RawValue::from_string(string).unwrap();

  // from here we know json.len() > 1 because an empty string is not a valid json value.
  let json = raw.get();
  let first = json.as_bytes()[0];

  #[cfg(debug_assertions)]
  if first == b'"' {
    assert!(
      json.len() < MAX_JSON_STR_LEN,
      "passing a string larger than the max JavaScript literal string size"
    )
  }

  let return_val = if json.len() > MIN_JSON_PARSE_LEN && (first == b'{' || first == b'[') {
    let serialized = Serialized::new(&raw, &options).into_string();
    // only use JSON.parse('{arg}') for arrays and objects less than the limit
    // smaller literals do not benefit from being parsed from json
    if serialized.len() < MAX_JSON_STR_LEN {
      cb(&serialized)
    } else {
      cb(json)
    }
  } else {
    cb(json)
  };

  Ok(return_val)
}

// Formats a function name and argument to be evaluated as callback.
//
// This will serialize primitive JSON types (e.g. booleans, strings, numbers, etc.) as JavaScript literals,
// but will serialize arrays and objects whose serialized JSON string is smaller than 1 GB and larger
// than 10 KiB with `JSON.parse('...')`.
// See [json-parse-benchmark](https://github.com/GoogleChromeLabs/json-parse-benchmark).
pub fn format_callback<T: Serialize>(function_name: CallbackFn, arg: &T) -> Result<String> {
  serialize_js_with(arg, Default::default(), |arg| {
    format!(
      r#"
    if (window["_{fn}"]) {{
      window["_{fn}"]({arg})
    }} else {{
      console.warn("[TAURI] Couldn't find callback id {fn} in window. This happens when the app is reloaded while Rust is running an asynchronous operation.")
    }}"#,
      fn = function_name.0
    )
  })
}

// Formats a Result type to its Promise response.
// Useful for Promises handling.
// If the Result `is_ok()`, the callback will be the `success_callback` function name and the argument will be the Ok value.
// If the Result `is_err()`, the callback will be the `error_callback` function name and the argument will be the Err value.
//
// * `result` the Result to check
// * `success_callback` the function name of the Ok callback. Usually the `resolve` of the JS Promise.
// * `error_callback` the function name of the Err callback. Usually the `reject` of the JS Promise.
//
// Note that the callback strings are automatically generated by the `invoke` helper.
// pub fn format_result<T: Serialize, E: Serialize>(
//   result: Result<T, E>,
//   success_callback: CallbackFn,
//   error_callback: CallbackFn,
// ) -> crate::Result<String> {
//   match result {
//     Ok(res) => format_callback(success_callback, &res),
//     Err(err) => format_callback(error_callback, &err),
//   }
// }