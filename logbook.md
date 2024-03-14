# Logbook

- Can now emit events from the backend and listen to them in the frontend.
- Context menu shows up but clicks are not received at the backend.
- Remove punctuation from query before searching. Append `*` to the end of each word unless it is double-quoted.
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

# TODO

## Stage I: Indexing and Searching
> Stage I is a powerful desktop search tool. It will help people find documents and files more quickly on their computer. (Think of it as a replacement for Spotlight on Mac or Everything on Windows.)

### Database
- Restructure database:
  - Add `last_parsed` column to document table.
  - Add `folders` to the document table by splitting paths. Set extension as `"folder"` and `file_content` as None.
  - Create a new table for storing file_content in **chunks**. Connect it to the `document` table using a foreign key.
  - Keep **one FTS table** connected to the `text` table. No need for FTS table for `document`.
  - Rewrite the search query to use the new tables.

### Backend
- Allow searching by `created_at`, `last_modified` or `last_opened`.
- Implement `frecency`. Sort search results by `frecency`. Maybe weave `last_opened` into the formula?
- Figure out how to add app menu and context menus.
- Set up cron job to index every hour.
- Add graceful error handling instead of panic.
- Send data from backend to frontend ([see this](https://github.com/tauri-apps/tauri/discussions/7558))
- Check why QuickLook (`qlmanage`) blocks the main process. What if it was launched in a child process?
- Add a tray icon and menubar window.

### Front-end
- Connect loading spinners to the backend.
- Show feedback to the user about what dates are being parsed. Maybe as a popover button?
- Allow searching by created_at, last_modified or last_opened.
- Add icons for new file types.
- Disable default right-click context menu.
- Put double quotes on punctuation marks when cleaning query to make it work with the MATCH syntax