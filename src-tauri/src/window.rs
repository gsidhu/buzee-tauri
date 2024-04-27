use tauri::WebviewWindow;

pub fn hide_or_show_window(main_window: WebviewWindow) {
  // if main_window.is_visible().unwrap() {
  //   let _ = main_window.hide();
  // } else {
  let _ = main_window.show();
  let _ = main_window.set_focus();
  // }
}