<div align="center">
<img height=150 src="./static/Buzee Logo.png" />

<p align="center"><span>Full-text search app for Mac and Windows</span></p>

<div style="display: flex; flex-direction: row; justify-content: center; align-items: center; gap: 0.5rem;">
  <a href="/releases" style="all:unset; cursor: pointer; border: 1px solid #00a4ef; padding: 1rem; border-radius: 8px; display: flex; justify-content:center; align-items: center; gap: 0.5rem; background: #00a4ef; color: white;">
  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-windows" viewBox="0 0 16 16">
    <path d="M6.555 1.375 0 2.237v5.45h6.555zM0 13.795l6.555.933V8.313H0zm7.278-5.4.026 6.378L16 16V8.395zM16 0 7.33 1.244v6.414H16z"/>
  </svg>
  </svg> Download for Windows
  </a>
  <a href="/releases" style="all:unset; cursor: pointer; border: 1px solid black; padding: 1rem; border-radius: 8px; display: flex; justify-content:center; align-items: center; gap: 0.5rem; background: black; color: white;">
  <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-apple" viewBox="0 0 16 16">
    <path d="M11.182.008C11.148-.03 9.923.023 8.857 1.18c-1.066 1.156-.902 2.482-.878 2.516s1.52.087 2.475-1.258.762-2.391.728-2.43m3.314 11.733c-.048-.096-2.325-1.234-2.113-3.422s1.675-2.789 1.698-2.854-.597-.79-1.254-1.157a3.7 3.7 0 0 0-1.563-.434c-.108-.003-.483-.095-1.254.116-.508.139-1.653.589-1.968.607-.316.018-1.256-.522-2.267-.665-.647-.125-1.333.131-1.824.328-.49.196-1.422.754-2.074 2.237-.652 1.482-.311 3.83-.067 4.56s.625 1.924 1.273 2.796c.576.984 1.34 1.667 1.659 1.899s1.219.386 1.843.067c.502-.308 1.408-.485 1.766-.472.357.013 1.061.154 1.782.539.571.197 1.111.115 1.652-.105.541-.221 1.324-1.059 2.238-2.758q.52-1.185.473-1.282"/>
    <path d="M11.182.008C11.148-.03 9.923.023 8.857 1.18c-1.066 1.156-.902 2.482-.878 2.516s1.52.087 2.475-1.258.762-2.391.728-2.43m3.314 11.733c-.048-.096-2.325-1.234-2.113-3.422s1.675-2.789 1.698-2.854-.597-.79-1.254-1.157a3.7 3.7 0 0 0-1.563-.434c-.108-.003-.483-.095-1.254.116-.508.139-1.653.589-1.968.607-.316.018-1.256-.522-2.267-.665-.647-.125-1.333.131-1.824.328-.49.196-1.422.754-2.074 2.237-.652 1.482-.311 3.83-.067 4.56s.625 1.924 1.273 2.796c.576.984 1.34 1.667 1.659 1.899s1.219.386 1.843.067c.502-.308 1.408-.485 1.766-.472.357.013 1.061.154 1.782.539.571.197 1.111.115 1.652-.105.541-.221 1.324-1.059 2.238-2.758q.52-1.185.473-1.282"/>
  </svg> Download for Mac
  </a>
</div>

</div>

## What is Buzee?

<!-- ![Demo](./static/demo.gif) -->

Buzee (pronounced _boozey_) is a eight-year-old labrador retriever who can't play fetch but can love you like no other.

Buzee is also a full-text search application for your life. It helps you find your files, effortlessly.

Download v0.2.0 from the [Releases](/releases) page.

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
- Tauri v2 beta
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