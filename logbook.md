# Logbook

- Added non-document file types. Have to add front-end support for it.
- Fixed: Forbidden folders and unnecessary files are now ignored.
- Made global_shortcut and window hiding/showing work.
- Added QuickLook preview for Mac. Have to test for Windows.
- Setting allowed_filetypes in `indexing` and passing that wherever needed in backend or frontend.
- Created new module for text_extraction. Supports MS Office, `txt` and `md` files. PDFs are going to be difficult perhaps.
- Added Tauri commands for handling `O` and `Enter` keys.
- Loading recently opened files on startup.
- Date search added to backend.
- Added compromise to frontend. Works well but should add feedback to the user about what it is parsing. Maybe in light grey text under the search bar?
- Connect file indexing and search to front-end. Cleaning up electron IPC code remains.
- Counting the number of files added and letting front-end know when indexing is done.
- Made file indexing thread async using `tokio` crate.
- Storing UNIX timestamps as BIGINT/i64 instead of strings.
- Search with file type filter works.
- File search works.
- Turns out file watcher is not what I needed. I needed indexing. Added that. Documents are now getting added to the database.
- Added FTS table and triggers. + scaffolding code for file watcher
- Set up database and app folder in user's documents. Getting some sense of project structure. Feeling happy.
- Learning from Archive_Cat and Orange to set up the database using `diesel` and a file watcher.
- Copied the `tauri.conf.json` from the Tauri 2.0 beta project to the SvelteKit project. Added the following dependencies to package.json –
```json
  "dependencies": {
    "@tauri-apps/api": ">=2.0.0-beta.0",
    "@tauri-apps/plugin-shell": ">=2.0.0-beta.0"
  },
  "devDependencies": {
    "@tauri-apps/cli": ">=2.0.0-beta.0",
  }
```
- On the side, set up a Tauri 2.0-beta project with vanilla HTML [using this guide](https://beta.tauri.app/guides/create/)
- Set up a barebones SvelteKit project with Tauri using [the quick start guide](https://tauri.app/v1/guides/getting-started/setup/sveltekit)
- Installed rust, rustup, cargo, and rustc using [the prerequisites guide](https://tauri.app/v1/guides/getting-started/prerequisites)

## TODO
- Figure out how to add app menu and context menus.
- Add a tray icon and menubar window.
- Set up cron job to index every hour.
- Add file content parsing.
- Send data from backend to frontend ([see this](https://github.com/tauri-apps/tauri/discussions/7558))
- Fix unnecessary file indexing. Ignore "Alias" and shortcut files.
- Add graceful error handling instead of panic.
- Disable right-click context menu in the webview.