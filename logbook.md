# Logbook

### Figure out how to stop the ThreadManager `handle` when clicked.

- Sync status is updated in the database each time it runs or app loads. Frontend also asks for sync status on each mount.
- Filetypes, user preferences and app data are now stored in the database. Declared new types/structs for filetypes. Next step: allow user to modify the filetypes and categories.
- Calling all SQLite PRAGMAS on each `establish_connection` call. Switched to WAL mode instead of journal. Hopefully this does not cause any database lock errors.
- Clearly segregating query into an object and generating the SQL query accordingly. Heavy robustness.
- Disabled OS spell check in the search bar because it kept making quotation marks weird. Will handle it through suggested search keywords later.
- Search is a lot more robust now. Negative-only queries are allowed. The placement of <query> <negative_query> <time> doesn't matter.
- Sync operation works. Updates status on the frontend as well.
- File content parsing works. Is a bit slow for Excel and PDF files though so keeping that off for now.
- File search also works. Amaze.
- Quicklook is opened in a new thread so that it doesn't block the main process.
- Recent documents load on the front-end perfectly.
- Figured out how to get results from the DB JOINing metadata with document table. Defined a new struct (`DocumentResponseModel`) for the results.
- Added command for detecting OS and sending it to the frontend.
- Onboarding works. Time to fix search now.
- Indexing metadata takes UNDER FIVE SECONDS because I removed my dumbass mistake from the for-loop.
- Figure out how to send files added incremental updates to front-end. SOLVED: just pass the Window variable from the command to the function.
- Indexing metadata takes about 90 seconds for all file types (~ 40,000 files).
- Added logging using the `log` and `simple_log` crates.
- Adding document metadata to the document table works. Metadata and metadata_fts tables get automatically updated using triggers.
- Created new database schema. Hopefully is more robust for future scale.
- Results row context menu options now work. It gets ping ponged between backend and frontend but have to use it till I figure out how to send data to the menuevent::receiver.
- Added command, types and function for DB Stats. The results are logged but don't render. The function is sloppy because of Rust's type system (couldn't use `group by` with `count`, instead calling a count statement for each file_type).
- Added page navigation functionality to context menu receiver using emit events.
- Context menu receiver is working. Now to add the functionality.
- Added infinite scroll to search results. The UI is a bit wonky.
- Loading 100 results on each search (in two pages).
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

# Notes
- [For notarization on MacOS, don't use relative paths and set all the variables in the .env file](https://discord.com/channels/616186924390023171/1217839724719509576)
- [Disable keyboard shortcuts on Windows webview2](https://discord.com/channels/616186924390023171/1126997012647264306/1128359828453085274)

# TODO

## Stage I: Indexing and Searching
> Stage I is a powerful desktop search tool. It will help people find documents and files more quickly on their computer. (Think of it as a replacement for Spotlight on Mac or Everything on Windows.)

### Database
- [x] Restructure database.
- Add user prefs to database.

### Backend
- Allow searching by `created_at`, `last_modified` or `last_opened`.
- Implement `frecency`. Sort search results by `frecency`. Maybe weave `last_opened` into the formula?
- [x] Figure out how to add app menu and context menus.
- Set up cron job to index every hour.
  - Add code for adding folders to the index.
  - Add code for indexing file contents.
- Add graceful error handling using logger instead of panic.
- [x] Send data from backend to frontend ([see this](https://github.com/tauri-apps/tauri/discussions/7558))
- [x] Check why QuickLook (`qlmanage`) blocks the main process. What if it was launched in a child process?
- Add a tray icon and menubar window.
- Figure out how to pass types to tauri commands.
- Add search suggestions.
- Let front end know when app is not in focus so that it can change CSS.

### Front-end
- Connect loading spinners during onboarding, sync and search to the backend.
- Show feedback to the user about what dates are being parsed. Maybe as a popover button?
- Allow searching by created_at, last_modified or last_opened.
- Add icons for new file types.
- Disable default right-click context menu.
- Put double quotes on punctuation marks when cleaning query to make it work with the MATCH syntax
- [x] Display counter of number of files logged when indexing during onboarding.
- Add a Preview sidebar that shows a thumbnail and complete metadata of the selected file.
- Allow user to add/remove file types and categories.
- Update Document Stats to reflect categories and # files indexed.