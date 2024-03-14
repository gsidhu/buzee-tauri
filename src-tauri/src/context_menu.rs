// use tauri::menu::{Menu, MenuItem, Submenu};

// pub fn initialize() {
//   // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
//   let quit = CustomMenuItem::new("quit".to_string(), "Quit");
//   let close = CustomMenuItem::new("close".to_string(), "Close");
//   let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
//   let menu = Menu::new()
//     .add_native_item(MenuItem::Copy)
//     .add_item(CustomMenuItem::new("hide", "Hide"))
//     .add_submenu(submenu);
// }

// use tauri::menu::MenuBuilder;

// tauri::Builder::default()
//     .setup(|app| {
//         let menu = MenuBuilder::new(app)
//             .copy()
//             .paste()
//             .separator()
//             .undo()
//             .redo()
//             .text("open-url", "Open URL")
//             .check("toggle", "Toggle")
//             .icon("show-app", "Show App", app.default_window_icon().cloned().unwrap())
//             .build()?;
//         Ok(())
//     })

use crate::custom_types::Payload; // Import the Error type
use tauri::{
  menu::{Menu, MenuItem},
  Manager, Window,
};

pub fn searchresult_context_menu(window: &Window) {
  let manager = window.app_handle();
  let context_menu = Menu::with_items(manager, &[
    #[cfg(target_os = "macos")]
    &MenuItem::with_id(manager, "preview", "Show Preview", true, None::<&str>).unwrap(),

    &MenuItem::with_id(manager, "open", "Open File", true, None::<&str>).unwrap(),
    &MenuItem::with_id(manager, "open_folder", "Open Folder", true, None::<&str>).unwrap(),
    &MenuItem::with_id(manager, "copy", "Copy File Name", true, None::<&str>).unwrap(),
    &MenuItem::with_id(manager, "copy_path", "Copy File Location", true, None::<&str>).unwrap(),
  ]).unwrap();

  window.popup_menu(&context_menu).unwrap();
}

pub fn statusbar_context_menu(window: &Window) {
    let manager = window.app_handle();
    let context_menu = Menu::with_items(
        manager,
        &[
          &MenuItem::with_id(manager, "document_stats", "Document Stats", true, None::<&str>).unwrap(),
          &MenuItem::with_id(manager, "deep_breathing", "Deep Breathing", true, None::<&str>).unwrap(),
          &MenuItem::with_id(manager, "tips_and_shortcuts", "Tips && Shortcuts", true, None::<&str>).unwrap(),
          &MenuItem::with_id(manager, "quit", "Quit", true, None::<&str>).unwrap()
        ],
    )
    .unwrap();

    window.popup_menu(&context_menu).unwrap();
}

// use muda::MenuEvent;
// Use the MenuEvent::receiver to listen to click events on the menu items
pub fn initialize_receiver(window: &Window) {
  // window.listen(event, move |event| {
  //   println!("window just loaded a component");
  //   let menu_channel = MenuEvent::receiver();
  //   if let Ok(event) = menu_channel.try_recv() {
  //     println!("Received menu event: {:?}", event.id)
  //   }
  // });

  window.emit("menu-ready", Payload { message: "Yo whaddup?".into() }).unwrap();
}