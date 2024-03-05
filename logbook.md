# Logbook

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