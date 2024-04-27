# Logbook

Note: Adding both textra and winocr binaries as sidecar does not build. Need to provide only textra on Mac and only winocr on Windows.

- Add a page to let user OCR a PDF or image.
  - Add dialog to select file.
  - Add "save to database" toggle.

## - Add UI to OCR PDF Image.
## - Use GitHub action to build. Figure out how to package sidecars for specific OS.
## - Add ignore paths to indexing functions. Put {app_directory}/temp_output.txt in it.
## - Figure out how to make AltGr work same as Alt on Windows.

## - Check why app updater does not restart the app.
## - Check if app updater works on Windows.

- Added UI for ignoring files/folders.
- User can now add files or folders from network/external drives from the Settings.
- Calling the GlobalShortcut will now only _show_ the window. Not hide it. If user wants to hide, they can press the minimise button or move to another app/screen.
- Fixed Svelte store variables for setting StatusBar items. Setting UserPreferences in the store on app load.
- Changed indexing/scan functions so they take a file_path array instead of a single file_path. This will allow user to select network/external drives from the Settings.
- Split DocumentItem creation out of the monolith walk_directory function.
- Added app updater.
  - Created public/private signing keys for the updater. Password stored in Bitwarden.
  - Added `plugin-updater` and `plugin-dialog` to the project.
  - Added 'Check for Updates' button in the settings page.
  - Checks for app updates on each app load. (in `+layout.svelte`)
- Fixed edge case when setting empty global shortcut. Or setting both modifier keys as the same. Showing error as well.
- Tested global shortcut setting UI on Windows. Works.
- Using modified textra binary. Set the activation policy to accessory. Now extra app icon does not show up. [Discussion here](https://discord.com/channels/616186924390023171/1230776346767003678/1231205322845196288)
- Add UI for setting global shortcut. Works on Mac. Windows needs to be tested.
- Also added a switch_off flag to the sync process so repeat requests are ignored.
- Keeping the background sync button in the statusbar disabled for as long as the sync process doesn't stop. This is to prevent multiple sync processes from running at the same time.
- Made Windows OCR work. Downloads Poppler if doesn't exist. Fixed winocr_cli to make things work.
- Added Windows OCR sidecar.
- Reorder indexing so all regular docs are indexed first, then PDFs (then eventually, XLSX and CSV).
- Added MacOS code signing and notarization. Have to export env variables to shell before running `tauri build`. Or use the GitHub action.
- Connected settings to backend. Detailed Scan, Automatic Scan, Global Shortcut settings are respected.
- Using `scan` instead of sync in all user-facing copy.
- Showing brief message in status bar to tell user what the app is doing. E.g. "restarting" or "starting/stopping scan"
- Change `sendEvent` to `sendEventToFirebase`.
- Show size instead of last_opened in results table.
- Moved StatusBar to +layout.svelte to show it across the app.
- Added graceful_restart function that sets the sync_running flag to false, waits for 3 seconds, then restarts the app.
- Created a user pref Mutex State. Merge global shortcut state into it. Pull it from the DB on app start. Send it to the frontend when showing the settings page. Change value in both DB and Mutex when user makes changes in the settings page.
- Added functions/commands for toggling the global shortcut.
- Showing a little checkmark next to the filename for all files that have been indexed.
- Added last_parsed column to the document table. Modified all models, schemas and functions accordingly.
- Modified CSV and XLSX parser to store only unique words.
- Removed all unused imports and functions.
- Replaced function for handling not-query only. Should work more reliably now.
- Added cron job. Majestic.
- Sorting files by size for indexing.
- Storing sync_running flag in mutex state. Seems to run quite well. More reliable than DB flag.
- Storing connection pool in mutex state.
- Reading `output_text` file only when the PDF extraction process completes.
- Using diesel::r2d2 to manange a connection pool so db does not lock. 
- Replaced extraction hashmap with async/await to make PDF extraction work.
- Fixed textra installation script. Using `curl` instead of `wget`.
- Removed unnecessary `docx` dependency. Using `dotext` for extracting docx, pptx and xlsx anyway.
- Added a spare function for reading large txt files.
- Added PDF file parsing using [textra](https://github.com/freedmand/textra). Textra is automatically installed in the app_dir when sync runs for the first time. On each sync run, it is checked whether textra is installed and available.
- Disabled Sentry and Firebase for now. Windows gave CORS issue and wouldn't load. On Mac, sentry is recording needlessly.
- Removed code for border-less window.
- Added support for opening file previews in QuickLook on Windows. TODO: Add explanation on how to install and setup QuickLook.
- Disabled default right click context menu using JavaScript in `+layout.svelte`.
- Added code blocks to make app build on Windows. Localhost IPC CORS issue remains to be resolved.
- Can set new global shortcut from the frontend. Requires app restart. Works perfectly. (on build, not dev)
- Set up a mutable state to store the global shortcut string instead of making DB calls each time. Can use [this guide](https://tauri.by.simon.hyll.nu/concepts/tauri/state_management/) for storing auth tokens in future.
- Global launch shortcut is set from the string stored in the `user-preferences` table in the DB.
- Added event listener to detect when window loses focus to grayscale the window contents.
- Disabled body scroll on results page, enabled elsewhere.
- Replace popover with title attribute on result path. Added it to name column as well.
- Fixed delete triggers in database.
- Added icons using `npm run tauri icon`.
- Added `folder` as a filetype dropdown.
- Suggested search works.
- Populating search suggestions from the database combining items from file contents, file names and the complete filename itself.
- Cleaned up unnecessary console logs.
- Added UI and interactions for search suggestions.
- Disabling sync button in status bar for 10 seconds on each click. Showing appropriate UI feedback. This is to allow the sync operation to start properly or complete ongoing tasks before the user can click again.
- Folder search enabled. Added folders to the metadata and _fts tables as `extra_tag`. There will always be some column specific to a source (like file_type for documents or subject for emails) that needs to be indexed. This is a simple way to do it.
- Modified sync function to use WebviewWindow instead of Window. So that it can be called from the tauri builder function.
- Now highlighting the file when opening the folder. Fallback to just opening the folder.
- Fixed global shortcut.
- Made search bar file_type dropdown work. Storing allowedExtensions in the store now.
- Getting up to 100 results on each search since results from both FTS tables are combined. This is good because then we don't need to run double searches from the front end.
- NOT query case handled as before. Not using body_fts for that because it is going to be quite impossible for a user to spot a document that has the search query _inside_ it but not in its title. Marginal utility is low.
- Combining results from body_fts and metadata_fts and sorting them by last_modified. Eventually will sort them by frecency_rank. This also solves the problem of showing folders and non-document files in search results (since they are not included in body_fts, they come from metadata_fts).
- Added body_fts search query. Broke down the fts_search function into smaller components.
- Optimised file content parsing. Runs _a lot_ faster now.
- Removed ThreadManager because it didn't work as intended.
- Reduced new database connection requests.
- Added MOBI file parsing but not sure if it works yet. Created issue on GitHub repo.
- Added EPUB file parsing.
- ThreadManager stores the `handle` when sync operation starts. The `app_data` table in the DB sets the sync_running flag to true. The parse_file function checks this flag at the end of the loop. If it is true, it waits for 5 seconds and then checks again. If it is false, it continues. The sync operation sets the flag to false when user clicks a second time or when the operation is completed. Not the most glorious way but this is what worked.
  - Can't use a global variable using Arc and Mutex because the tauri command and the file parse function get separate handles on the thread manager; so updating the kill_flag in one doesn't affect the other.
  - Have to pass a receiever to the file parse function but that just doesn't seem to work.
  - For now, updating the DB flag it is, until I find a better working solution.
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
- [x] Add user prefs to database.

### Backend
- [x] Add search suggestions.
- Handle large file parsing.
  - Make XLSX, CSV and PDF indexing better.
  - [x] Add PDF parsing on Windows.
  - Add image text parsing.
  - Set up separate child processes for parsing (1) regular documents (2) pdfs and images (3) xlsx and csv.
    - Dividing into tokio threads makes the app panic quit when running read operations. Tried using a connection pool and it didn't help either. So, sticking to a single thread for all types of files for now. Maybe sort them by file size and run the smallest files first?
  - [x] Use state management for storing reference to child processes. Use single sync button on the front end.
- Detect paths/files in OneDrive, Google Drive, Dropbox, etc. and ignore them.
- Add graceful error handling (`unwrap_or`) using logger instead of panic (esp. on `unwrap` values).
- [x] Clean up the connection requests using the pool.
- Figure out how to pass types to tauri commands.
- Add logging to a file + sentry/firebase events.
- ? Add a tray icon and menubar window.
- ? Implement `frecency`. Sort search results by `frecency`. Maybe weave `last_opened` into the formula?
- ? Maybe use a separate index for search suggestions. It makes too many queries to the SQLite db.
- Allow searching by `created_at`, `last_modified` or `last_opened`.
- [x] Figure out how to add app menu and context menus.
- [x] Add code for indexing file contents.
- [x] Send data from backend to frontend ([see this](https://github.com/tauri-apps/tauri/discussions/7558))
- [x] Check why QuickLook (`qlmanage`) blocks the main process. What if it was launched in a child process?
- [x] Optimise file indexing. 
  - [x] Check why it seems to be stuck sometimes. ANS: multiple select calls to db were the bottleneck.
  - [x] Also check why the kill switch doesn't work sometimes. (It seems the database flag is not being updated when the sync operation completes.) ANS: sync_status wasn't being updated inside the for-loop.
- [x] Let front end know when app is not in focus so that it can change CSS.

### Front-end
- Connect loading spinners during onboarding, sync and search to the backend.
- ? Show feedback to the user about what dates are being parsed. Maybe as a popover button?
- [x] Add icons for new file types.
- Disable default right-click context menu.
- [x] Put double quotes on punctuation marks when cleaning query to make it work with the MATCH syntax
- [x] Display counter of number of files logged when indexing during onboarding.
- ? Add a Preview sidebar that shows a thumbnail and complete metadata of the selected file.
- Allow user to add/remove file types and categories.
- Update Document Stats to reflect categories and # files indexed.
- Add infinite scroll to search results. (Need Punnu's help for this.)
- Fix window overflow issue.