use crate::custom_types::Payload;
use crate::housekeeping::{get_documents_directory, APP_DIRECTORY};
use crate::utils::norm;

// Import the Error type
use tauri::{
  menu::{Menu, MenuEvent, MenuId, MenuItem}, Manager, Window
};

pub fn searchresult_context_menu(window: &Window) {
  let manager = window.app_handle();
  let context_menu = Menu::with_items(manager, &[
    #[cfg(target_os = "macos")]
    &MenuItem::with_id(manager, "preview", "Show Preview", true, None::<&str>).unwrap(),

    &MenuItem::with_id(manager, "open", "Open File", true, None::<&str>).unwrap(),
    &MenuItem::with_id(manager, "open_folder", "Open Folder", true, None::<&str>).unwrap(),
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
          &MenuItem::with_id(manager, "app_folder", "Show App Folder", true, None::<&str>).unwrap()
        ],
    )
    .unwrap();

    window.popup_menu(&context_menu).unwrap();
}

// Use the MenuEvent::receiver to listen to click events on the menu items
pub fn contextmenu_receiver(app: &tauri::AppHandle, event: MenuEvent) {
  // Define a trait to convert the MenuId to a string
  impl MenuIdString for MenuId {
    fn to_string(&self) -> String {
      self.0.to_string()
    }
  }
  trait MenuIdString {
    fn to_string(&self) -> String;
  }

  let event_id_string: &str = &event.id().to_string();

  match event_id_string {
    "open" => {
      app.emit("open", Payload { message: "Open File".into(), data: "".into() }).unwrap();
    }
    "open_folder" => {
      app.emit("open-folder", Payload { message: "Open Folder".into(), data: "".into() }).unwrap();
    }
    "app_folder" => {
      let app_dir_path = format!("{}/{}", get_documents_directory().unwrap(), APP_DIRECTORY);
      let app_dir_path = norm(&app_dir_path);
      let _ = open::that_detached(app_dir_path);
    }
    "document_stats" => {
      app.emit("document-stats", Payload { message: "Document Stats".into(), data: "".into() }).unwrap();
    }
    "deep_breathing" => {
      app.emit("deep-breathing", Payload { message: "Deep Breathing".into(), data: "".into() }).unwrap();
    }
    "tips_and_shortcuts" => {
      app.emit("tips-and-shortcuts", Payload { message: "Tips & Shortcuts".into(), data: "".into() }).unwrap();
    }
    #[cfg(target_os = "macos")]
    "preview" => {
      app.emit("show-preview", Payload { message: "Show Preview".into(), data: "".into() }).unwrap();
    }
    _ => println!("Invalid context menu option"),
  }
}