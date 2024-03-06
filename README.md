# Buzee - Tauri
Rust implementation of Buzee. Hopefully it'll be faster, lighter and more reliable.

## App Model
- App Data in Documents
  - Contains Database
    - Database has one table and one virtual table (for FTS)
    - Triggers are set between the table and virtual table for CRUD operations
  - Define types for document items/results
  - File watcher
    - Adds all allowed file types into the database (maybe add 'folder' as a filetype so it can fit into the same tables?)
    - Sets up a watcher that detects file changes and keeps the database updated
  - File search
    - Commands trigger FTS searches
- Add app tray icon and its interplay with window
- On first load
  - User runs through onboarding and gives permission to scan full disk
  - File watcher

- Run `compromise` on the search query in the front-end.
  - Any other necessary npm packages could be added to the front-end as well.