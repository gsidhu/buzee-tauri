# Buzee - Vision and Roadmap
One more app to reduce the clutter of all the others.

> This is Bob. Bob uses Gmail, Outlook, Dropbox, Reddit, Discord, Slack, WhatsApp, Spotify, YouTube, Obsidian, Amazon, Goodreads, and Google Scholar – all in a single day. When Bob loses a link, a document, or a note, he wishes his brain had more RAM.
>
> Bob wishes there was a way to search all his apps from one place. Bob wishes there was a way to save and organize the important bits from them. Bob wishes there was a way to keep his context intact. Bob wishes there was Buzee.

We couldn't find Bob, but we built Buzee for him. And for you.

## What is Buzee?
Pick your tagline:
1. One search to rule them all. 
2. RAM for your brain.
3. Work faster, smarter, and better.

Access your files, browser history, bookmarks, notes, emails - [everything](#connectors) - from one place.

### Features
1. **One Search**: Search across all your data sources from one place. Filter as you like and get results instantly.
2. **Personal Library**: Save anything you find important. A photo, video, podcast episode, article, book, song, note, email - anything.
3. **Robust Archive**: Your saved data stays with you. No more broken links, deleted content, or lost context.
4. **Work Offline**: Search your personal library even without an internet connection.
5. **Privacy First**: Everything runs and gets stored on your device. No data is sent to any server.
6. **Modular and Extensible**: Only use the connectors that you need. Add more as you like.
7. **Open Source**: Buzee is open source. You can see how it works and contribute to it.

## ★ Buzee Freeze: Never Lose Context
Whether you are a student, a researcher, a developer, a designer, or a manager, you work on several tasks. But you don't work on them all at once. Sometimes you need to pause work and come back to it later.

Working on any task requires you to juggle between multiple apps, websites, and documents. The only thing holding it all together is your brain. This can be overwhelming and time-consuming.

This is what Buzee Freeze helps you with.

With Buzee Freeze, you create _Boxes_ of context. Each _Box_ represents a task. Each task may contain whatever documents, emails, notes, links, media, and apps you need to complete it.

When you need to pause work, you **freeze** the Box to save your mental and digital state. When you return, you **unfreeze** (or _defrost_?) the Box to pick up right where you left off. Hit the ground running every time.

Buzee Freeze provides you the contextual cues of a physical workspace in a digital environment. It is your personal library, your digital workspace, and your mental context all in one.

## What Buzee is and isn't
1. **Buzee is not a replacement for your existing apps. It is a companion to them.** You will still use your email app, your note-taking app, your cloud storage, your messaging apps, your browser, your media player, and your code editor. Buzee will just help you access and search through all of them from one place. It will help you save and organize the important bits from them. It will help you keep your context intact.
2. **Buzee is meant to be fast, simple and compatible.** You can use it on your Mac or Windows computer. You can use it offline and you can use it with whatever apps and services you already use.
3. **Buzee breaks silos and makes your data accessible.** A tangential problem that Buzee solves is that data is often locked inside apps. You can't search your emails from your note-taking app. You can't search your notes from your email app. You can't search your cloud storage from your messaging app.

### Does Something Like Buzee Exist?
Yes, and no.

If you have used a universal launcher like [Raycast](https://raycast.com/), or an app like [Readwise Reader](https://readwise.io/read) that ties your articles, newsletters and PDFs together, you have seen a glimpse of what Buzee can do. But Buzee is not just a launcher or a reader.

Another closely related software is [ArchiveBox](https://archivebox.io/) but it leans more towards archiving webpages and media, and less towards searching and organizing your personal data.

What sets Buzee apart is that it is privacy-first and prioritises desktop and offline usage. It is a desktop app (a software, as we used to call them) that runs and stores all data on your device. No data is sent to any server.

It may seem that apps like Notion, Obsidian, Anytype or Capacities are similar to Buzee. But they are not. They are note-taking or personal knowledge management apps. They are not designed to be a search tool for all your data. You can't search your emails, your cloud storage, your browser history, or your media library from them. Plus, most of these apps are cloud-first. 

Most importantly, these apps position themselves as an all-in-one _replacement_ for your existing apps. Buzee is not a replacement, it is a companion.

### Why no AI or Collaboration features?
Generative AI or AI-powered classification of unorganised data are hard problems when it comes to privacy, quality, and personalisation. It would be ideal if Buzee could –
* Automatically classify and tag your data. For e.g. to create profiles of your projects or clients.
* Generate templates and summaries from your data. For e.g. to create reports or presentations.

But the trade-offs are not worth it. Buzee is not a data science project.

Similarly, collaboration features like sharing, commenting, and editing are beyond the current scope. Such features are better suited for dedicated apps with specific focus like Google Docs, Notion, or Slack.

However, Buzee is designed to be extensible. It will have a plugin system that could potentially allow AI and collaboration features.

## Roadmap
Buzee is in its early stages. The dev work is planned in the following stages:
1. Stage I: Indexing and Searching
2. Stage II: Plugin System and Connectors
3. Stage III: Buzee Freeze

We are currently approaching completeness of Stage I.

The idea is to make each stage a compelling product on its own. For example: 
* Stage I is a powerful desktop search tool. It will help people find documents and files more quickly on their computer. (Think of it as a replacement for Spotlight on Mac or Everything on Windows.)
* Stage II will be a modular and extensible search tool. Beyond files, users will be able to search across their apps, websites, and services. The plugin system opens up possibilities of tools beyond search like calculators, notes, timers, shortcuts, converters, etc. (Think of it as a replacement for Spotlight or Raycast on Mac.)
* Stage III will be a digital workspace and context manager. For knowledge workers, it will help them keep their context intact across tasks and time. (Think of it as a digital equivalent of a physical desk, workbench, or pinboard.)

### Tech Stack
Buzee is built using the following technologies:
1. **Frontend**: SvelteKit with Typescript
2. **Backend**: Rust
3. **Desktop App**: Tauri
4. **Database**: SQLite
5. **Search**: FTS5

Wherever possible, Buzee will leverage native APIs and system features to provide a seamless experience.

### Connectors
* **Local Files**: Personal documents, Work documents, External storage, Code files, Music, Images, Videos, Movies, etc.
* **Cloud Storage**: Google Drive, Dropbox, OneDrive, etc.
* **Messaging Apps**: WhatsApp, Telegram, Slack, Messenger, Instagram, etc.
* **Forums**: Reddit, Hacker News, Discord, etc.
* **Browser History**: Chrome, Firefox, Safari, Arc, etc.
* **Bookmarks**: Browser Bookmarks, Pocket, Instapaper, Omnivore, Readwise, Pinboard, etc.
* **Media Library**: Spotify, Apple Music, YouTube Music, YouTube, Podcasts, etc.
* **Notes**: Evernote, Notion, Roam, Obsidian, Bear, Mem, etc.
* **Emails**: Gmail, Outlook, etc.
* **Research**: Google Scholar, JSTOR, Zotero, Mendeley, etc.
* **Code**: GitHub, GitLab, Bitbucket, etc.
* **Health**: Apple Health, Google Fit, Fitbit, Nike Run Club, Strava, Garmin, Coros, Endomondo etc.
* **Calendar**: Google Calendar, Apple Calendar, etc.
* **To-Do**: Todoist, Things, Trello, Asana, etc.
* **Shopping**: Amazon, eBay, Etsy, etc.
* **Books**: Kindle, Audible, Goodreads, Apple Books, etc.
* **Sites and Feeds**: RSS Feeds, Newsletters, Blogs, Wikipedia, etc.

### Pitch
Buzee – the ultimate local-first, privacy-first personal data search engine. Buzee seamlessly integrates with all your data sources – from local files to cloud storage, messaging apps, forums, emails, and beyond. Using advanced natural language processing, Buzee allows you to search your entire digital footprint effortlessly. Find your documents, track your health stats, recall your messages, and much more with a simple, secure search.

Imagine having all your digital life at your fingertips, without compromising your privacy. That's what Buzee does for you. It's a powerful search engine that connects to everything – your local files, cloud storage, emails, chats, social media, music, and even your fitness apps. Want to know your average running pace from July 2019? Or find that tech article you saved in 2020? Buzee makes it possible. With its natural language search, you can effortlessly sift through your entire digital history. It's like having a super-organized, private assistant for all your online and offline data.