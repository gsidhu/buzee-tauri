<div align="center">
<img height=150 src="./static/Buzee Logo.png" />

<p align="center"><span>Full-text search app for Mac and Windows</span></p>

Download the latest release from the [Buzee website](https://buzee.co/).
</div>

> ⚠️ I have _just_ open sourced Buzee. The documentation is lacking. Please open an issue and I'd be happy to help! ⚠️

## What is Buzee?

Buzee (pronounced _boozey_) is a eight-year-old labrador retriever who can't play fetch but can love you like no other.

Buzee is also a full-text search application for your life. It helps you find your files, effortlessly.

Download v0.2.0 from this [Releases](https://github.com/gsidhu/buzee-releases/releases) page.

## Screenshots

![Screenshot 1](./static/Screenshot1.png)
![Screenshot 2](./static/Screenshot2.png)
![Screenshot 3](./static/Screenshot3.png)

## Features
- Fast, full-text search for all your documents, images, audio, video, folders, and browser history.
- Search all local documents and folders by keyword, time, type, or any combination of these.
- Ignore specific files or folders from being indexed. Or ignore only their content.
- View search results in a list or icon view.
- View statistics about your files.
- Sub-features:
  - Extract text from PDFs and Images using OCR.
  - Use a Scratch Pad to quickly jot down notes.
- Automatically syncs with changes on your filesystem.
- Lightweight installation package and low memory usage.
- Supports these default file types: 
```
Documents: csv, docx, key, md, numbers, pages, pdf, pptx, txt, xlsx, xls
Images: jpg, jpeg, png, gif
Books: epub, mobi, azw3, pdf
Audio: mp3, wav, aac, flac, ogg
Video: mp4, mkv, avi, mov, wmv
```

## Building from Source
Buzee works best on Mac. Windows may throw up some issues because I haven't had a change to properly test it. Linux is untested entirely, so you're on your own there.

1. Clone the repository.
2. Install Rust and NodeJS.
3. Run `npm install` in the root directory.
4. Run `cargo install` in the `src-tauri` directory.
5. Run `npm run tauri dev` in the root directory to run the app in development mode.
6. Run `npm run tauri build` in the root directory to build the app for production.

## TODO / Known Issues
```
Index:
(~) : partly implemented
(+) : has to be built from scratch
(?) : not sure if it will add great value
```

- (~) Show matching text for search results by reading from the `body` table.
- (~) Browser history search should support complex queries the way document search does.
- (~) Icon view should load thumbnails in an efficient, non-blocking manner. Thumbnails should show up on the page as they are loaded.
- (~) Enable adding 'comments' to documents.
- (~) Enable pinning documents/folders to the top of search results.
- (~) Allow user to add or remove supported file types.
- (~) Allow user to switch between profiles on Arc and Chrome. (Currently uses the default profile)
- (~) Test for Linux.
- (~) Improve the speed of parsing PDFs, Images and XLSX files. Especially OCR operations.
- (+) Enable adding 'tags' to documents.
- (+) Create a 'Dashboard' view that shows statistics, pinned documents, and recent searches.
- (+) Add tests to the codebase.
- (?) Record frecency of documents and use it to sort search results.

## Tech Stack
### Dependencies
Back-end:
- Rust 
- Tauri v2
- SQLite
- Tantivy

Front-end:
- Svelte 4 using TypeScript
- shadcn-svelte
- TailwindCSS

See all dependencies in the [Cargo.toml](/src-tauri/Cargo.toml) and [package.json](/package.json) files.

### Architecture
- All file metadata is stored in SQLite in the `document` table. A central `metadata` table stores the metadata from files and eventually cloud services, emails etc. 
- A full-text index is created on `metadata` and stored as the `metadata_fts` table.
- Parsed text from documents is stored in the `body` table.
- A full-text index is created in Tantivy at the same time.
- The Firefox, Chrome and Arc history is searched using their respective history databases directly.
- All front-end code is in the `src` directory. All back-end code is in the `src-tauri` directory.

## Contributing / Way Forward
Read the [Vision and Roadmap](./VISION&ROADMAP).

I have spent two years building this project. It started as an Electron app, then I switched to Tauri for performance gains. When I started I barely new JavaScript and Svelte. Over the course of development, I learned NodeJS, TypeScript, SQLite, Rust, Tauri, Tantivy, and many other technologies. I learned so much about managing a project of this size and complexity. I am proud of what I have built but I am more proud of what I have learned.

I am now letting go of this project because I have other priorities. Please feel free to do with this project as you wish. I am happy to help you get started with the codebase.

If nothing else, this project can serve as an example of how to build a full-text search engine using Tauri and Tantivy. There are several tiny features and performance workarounds that I have implemented that you might find useful.

If you do do something with this project, please let me know. I would love to see what you build!

## License
MIT