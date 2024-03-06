// use crate::database::models::DocumentItem;
// use crate::database::crud;
// use crate::utils::{list_dir_contents, get_metadata};
// use std::path::Path;
// use std::sync::mpsc::channel;
// use notify::{raw_watcher, Op, RawEvent, RecursiveMode, Watcher};
// use chrono::NaiveDateTime;

// const ALLOWED_FILETYPES: [&str; 12] = ["csv", "docx", "key", "md", "numbers", "pages", "pdf", "pptx", "txt", "xlsx", "xls", "folder"];

// pub struct FsWatcher {
//   path: String,
// }

// impl FsWatcher {
//   pub fn new(path: String) -> FsWatcher {
//     FsWatcher { path }
//   }

//   pub fn start(&mut self) {
//     let (tx, rx) = channel();
//     let mut watcher = raw_watcher(tx).unwrap();
//     let wt_res = watcher.watch(self.path.as_str(), RecursiveMode::Recursive);
//     if wt_res.is_err() {
//       println!("{:?}", wt_res.err());
//       println!("watch {} err ", self.path);
//       return;
//     }
//     println!("fs watcher started for {}", self.path);

//     loop {
//       println!("waiting for events");
//       match rx.recv() {
//         Ok(RawEvent {
//           path: Some(path),
//           op: Ok(op),
//           cookie: _,
//         }) => {
//           let abs_path = path.to_str().unwrap().to_string();
//           let path = Path::new(&abs_path);

//           println!("{}: {:?}", abs_path, op);
          
//           let metadata = get_metadata(path).unwrap();

//           if metadata.is_symlink() {
//             continue;
//           }

//           if Op::REMOVE & op == Op::REMOVE {
//             crud::delete_file_from_database(abs_path);
//           } else {
//             let filename = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
//             let mut extension = path.extension().and_then(std::ffi::OsStr::to_str);
//             let is_folder = metadata.is_dir();
//             if is_folder {
//               extension = Some("folder");
//             }
//             let filesize = metadata.len();
//             // convert last_modified, last_opened and created_at to NaiveDateTime
//             let last_modified = NaiveDateTime::from_timestamp_opt(metadata.modified().unwrap().elapsed().unwrap().as_secs() as i64, 0);
//             let last_opened = NaiveDateTime::from_timestamp_opt(metadata.accessed().unwrap().elapsed().unwrap().as_secs() as i64, 0);
//             let created_at = NaiveDateTime::from_timestamp_opt(metadata.created().unwrap().elapsed().unwrap().as_secs() as i64, 0);
            
//             // If ALLOWED_FILETYPES does not contain `extension` or if path.is_dir(), continue
//             if !ALLOWED_FILETYPES.contains(&extension.unwrap()) || is_folder {
//               continue;
//             }

//             let file_item = DocumentItem::new(
//               filename,
//               &abs_path,
//               Some(filesize as f64),
//               extension.unwrap(),
//               None,
//               created_at.unwrap(),
//               last_modified.unwrap(),
//               last_opened.unwrap()
//             );

//             crud::add_file_to_database(file_item);
//           }
//         }
//         Ok(event) => {
//           println!("broken event: {:?}", event);
//           println!("event received but not handled: {:?}", event); // Debug print
//         }
//         Err(e) => {
//           println!("watch error: {:?}", e);
//           println!("error received: {:?}", e); // Debug print
//         },
//       }
//     }
//   }
//   fn save_subs(&mut self, parent_str: &str) {
//     let subs = list_dir_contents(parent_str);
//     for sub in subs {
//       let sub_path = Path::new(sub.as_str());

//       if let Ok(metadata) = sub_path.metadata() {
//         if metadata.is_symlink() {
//           continue;
//         }
//         let filename = sub_path.file_name().and_then(|s| s.to_str()).unwrap_or("");
//         let mut extension = sub_path.extension().and_then(std::ffi::OsStr::to_str);
//         let is_folder = metadata.is_dir();
//         if is_folder {
//           extension = Some("folder");
//         }
//         let filesize = metadata.len();
//         // convert last_modified, last_opened and created_at to NaiveDateTime
//         let last_modified = NaiveDateTime::from_timestamp_opt(metadata.modified().unwrap().elapsed().unwrap().as_secs() as i64, 0);
//         let last_opened = NaiveDateTime::from_timestamp_opt(metadata.accessed().unwrap().elapsed().unwrap().as_secs() as i64, 0);
//         let created_at = NaiveDateTime::from_timestamp_opt(metadata.created().unwrap().elapsed().unwrap().as_secs() as i64, 0);
        
//         // If ALLOWED_FILETYPES does not contain `extension` or if path.is_dir(), continue
//         if !ALLOWED_FILETYPES.contains(&extension.unwrap()) || is_folder {
//           continue;
//         }

//         let file_item = DocumentItem::new(
//           filename,
//           sub_path.to_str().unwrap(),
//           Some(filesize as f64),
//           extension.unwrap(),
//           None,
//           created_at.unwrap(),
//           last_modified.unwrap(),
//           last_opened.unwrap()
//         );

//         crud::add_file_to_database(file_item);
//       }
//     }
//   }
// }

// #[cfg(windows)]
// use crate::utils::get_win32_ready_drives;

// pub fn run_file_watcher() {
//   #[cfg(windows)]
//   win_run();

//   #[cfg(target_os = "macos")]
//   macos_run();
// }

// #[cfg(target_os = "macos")]
// fn macos_run() {
//   let mut watcher = FsWatcher::new("/Users/thatgurjot/Documents".to_string());
//   std::thread::spawn(move || {
//     watcher.start();
//   });
// }

// #[cfg(windows)]
// fn win_run() {
//   let drives = unsafe { get_win32_ready_drives() };
//   for driv in drives {
//     std::thread::spawn(move || {
//       let mut watcher = FsWatcher::new(driv);
//       watcher.start();
//     });
//   }
// }
