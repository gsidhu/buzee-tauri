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