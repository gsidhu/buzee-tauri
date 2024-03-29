/*
  SCHEMA LOGIC
  There are seven parts to the schema:
  1. `metadata`: A centralised metadata table that stores common metadata fields from all types of sources
  2. Individual tables for each data source that store common metadata + source-specific metadata (e.g. `document`, `email`, `bookmark`, `website` etc.)
  3. `body`: A table that stores content/body/text from all data sources in chunks. So text for each row in an individual table may be divided into multiple rows in this table.
  4. A FTS virtual table mapped to the `metadata` table.
  5. A FTS virtual table mapped to the `body` table.
  6. Triggers to keep the metadata and metadata_fts table updated when the source tables are updated.
  7. Triggers to keep the Body FTS virtual table updated when the body table is updated.

  Note: The search will run over the FTS tables.

  The source tables and the `body` table are updated from the app. The `metadata` table and the FTS virtual tables are updated using triggers only.

  Once other source tables are setup, perhaps individual FTS tables could be setup for each source table. This way the user can trigger a search over a specific domain.
*/

/*
  DOCUMENT TABLE
  source_domain = "local", "google_drive", "onedrive", "dropbox" etc.
  created_at = timestamp when the item was created on the source
  name, path, size, file_type, last_modified, last_opened : metadata of the document
  last_synced = timestamp when the item was last synced with the local database
  is_pinned = boolean to indicate if the document is pinned by the user in the app
  frecency_rank = float to indicate the frecency of the document
  frecency_last_accessed = timestamp when the document was last accessed using the app
  comment = user comment added in the app
  
  Note: cannot add metadata_id here because data is added to the `document` table first and then
  the metadata table gets automatically populated using triggers
*/
pub const DOCUMENT_TABLE_CREATE_STATEMENT : &str = r#"
  CREATE TABLE IF NOT EXISTS "document" 
  (
    "id" INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    "source_domain" TEXT NOT NULL,
    "created_at" BIGINT NOT NULL,
    "name" TEXT NOT NULL,
    "path" TEXT NOT NULL,
    "size" INTEGER,
    "file_type" TEXT NOT NULL,
    "last_modified" BIGINT NOT NULL,
    "last_opened" BIGINT NOT NULL DEFAULT 0,
    "last_synced" BIGINT NOT NULL DEFAULT 0,
    "is_pinned" BOOLEAN NOT NULL DEFAULT 0,
    "frecency_rank" REAL NOT NULL DEFAULT 0,
    "frecency_last_accessed" BIGINT,
    "comment" TEXT
  );
"#;

/*
  BODY TABLE (for all sources)
  metadata_id = id from the metadata table
  source_id = id from the source table (document, email, article, website etc.)
  text = body content of the document, email, article, website etc.
*/
pub const BODY_TABLE_CREATE_STATEMENT : &str = r#"
  CREATE TABLE IF NOT EXISTS "body" 
  (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    metadata_id INTEGER NOT NULL,
    text TEXT NOT NULL,
    title TEXT NOT NULL,
    url TEXT NOT NULL,
    last_parsed BIGINT NOT NULL DEFAULT 0,
    FOREIGN KEY (metadata_id) REFERENCES metadata(id)
  );
"#;

/*
  METADATA TABLE
  source_table = "document", "email", "bookmark", "website" etc.
  source_domain = "local", "google_drive", "dropbox", "gmail", "outlook", "pocket", "instapaper" etc.
  source_id = id from the source table (document, email, article, website etc.)
  title = title of the document, email, article, website etc.
  url = url or path
  created_at = timestamp when the item was created on the source
  last_modified = timestamp when the item was last modified on the source
  frecency_rank = float to indicate the frecency of the item
  frecency_last_accessed = timestamp when the item was last accessed using the app
  comment = user comment added in the app
  extra_tag = additional tag relevant to the source_table (e.g. file_type for document)
*/
pub const METADATA_TABLE_CREATE_STATEMENT : &str = r#"
  CREATE TABLE IF NOT EXISTS metadata (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    source_table TEXT NOT NULL,
    source_domain TEXT NOT NULL,
    source_id INTEGER NOT NULL,
    title TEXT NOT NULL,
    url TEXT NOT NULL,
    created_at BIGINT NOT NULL,
    last_modified BIGINT NOT NULL,
    frecency_rank REAL NOT NULL DEFAULT 0,
    frecency_last_accessed BIGINT,
    comment TEXT,
    extra_tag TEXT NOT NULL,
    FOREIGN KEY (source_id) REFERENCES document(id)
  );
"#;

/*
  METADATA FTS VIRTUAL TABLE
  All fields from the metadata table are added here but only title, url and comment are indexed
  Other (UNINDEXED) fields are added to reduce transactions for search
*/
pub const METADATA_FTS_VIRTUAL_TABLE_CREATE_STATEMENT : &str = r#"
  CREATE VIRTUAL TABLE IF NOT EXISTS metadata_fts 
  USING fts5(
    id UNINDEXED,
    source_table UNINDEXED,
    source_domain,
    source_id UNINDEXED,
    title,
    url,
    created_at UNINDEXED,
    last_modified UNINDEXED,
    frecency_rank UNINDEXED,
    frecency_last_accessed UNINDEXED,
    comment,
    extra_tag,
    content=metadata,
    tokenize="porter unicode61"
  );
"#;

/*
  BODY FTS VIRTUAL TABLE
  All fields from the body table are added here but only body is indexed
  metadata_id is kept for joining with the metadata table
  content is set to the `body` table
  tokenize is set to use the porter stemmer algorithm and unicode61 tokenizer (more suitable than ascii)
*/
pub const BODY_FTS_VIRTUAL_TABLE_CREATE_STATEMENT : &str = r#"
  CREATE VIRTUAL TABLE IF NOT EXISTS body_fts 
  USING fts5(
    metadata_id UNINDEXED,
    text,
    title,
    url,
    last_parsed UNINDEXED,
    content=body,
    tokenize="porter unicode61"
  );
"#;

/*
  METADATA TRIGGERS
  Triggers to keep the metadata table updated when the source tables are updated
*/
pub const TRIGGER_INSERT_DOCUMENT_METADATA : &str = r#"
  CREATE TRIGGER IF NOT EXISTS insert_document_metadata
  AFTER INSERT ON document
  BEGIN
      INSERT INTO metadata (source_table, source_domain, source_id, title, url, created_at, last_modified, frecency_rank, frecency_last_accessed, comment, extra_tag)
      VALUES ('document', NEW.source_domain, NEW.id, NEW.name, NEW.path, NEW.created_at, NEW.last_modified, NEW.frecency_rank, NEW.frecency_last_accessed, NEW.comment, NEW.file_type);
      INSERT INTO metadata_fts (source_table, source_domain, source_id, title, url, created_at, last_modified, frecency_rank, frecency_last_accessed, comment, extra_tag)
      VALUES ('document', NEW.source_domain, NEW.id, NEW.name, NEW.path, NEW.created_at, NEW.last_modified, NEW.frecency_rank, NEW.frecency_last_accessed, NEW.comment, NEW.file_type);
  END;
"#;
pub const TRIGGER_UPDATE_DOCUMENT_METADATA : &str = r#"
  CREATE TRIGGER IF NOT EXISTS update_document_metadata
  AFTER UPDATE ON document
  BEGIN
      UPDATE metadata
      SET source_domain = NEW.source_domain,
          source_id = NEW.id,
          title = NEW.name,
          url = NEW.path,
          created_at = NEW.created_at,
          last_modified = NEW.last_modified,
          frecency_rank = NEW.frecency_rank,
          frecency_last_accessed = NEW.frecency_last_accessed,
          comment = NEW.comment,
          extra_tag = NEW.file_type
      WHERE source_table = 'document' AND source_id = OLD.id;
      UPDATE metadata_fts
      SET source_domain = NEW.source_domain,
          source_id = NEW.id,
          title = NEW.name,
          url = NEW.path,
          created_at = NEW.created_at,
          last_modified = NEW.last_modified,
          frecency_rank = NEW.frecency_rank,
          frecency_last_accessed = NEW.frecency_last_accessed,
          comment = NEW.comment,
          extra_tag = NEW.file_type
      WHERE source_table = 'document' AND source_id = OLD.id;
  END;
"#;
pub const TRIGGER_DELETE_DOCUMENT_METADATA : &str = r#"
  CREATE TRIGGER IF NOT EXISTS delete_document_metadata
  BEFORE DELETE ON document
  BEGIN
      DELETE FROM body WHERE metadata_id = (SELECT id FROM metadata WHERE source_table = 'document' AND source_id = OLD.id);
      DELETE FROM metadata WHERE source_table = 'document' AND source_id = OLD.id;
      DELETE FROM metadata_fts WHERE source_table = 'document' AND source_id = OLD.id;
  END;
"#;


/*
  BODY FTS TRIGGERS
  Triggers to keep the Body FTS virtual table updated when the body table is updated
*/

pub const TRIGGER_INSERT_BODY_FTS : &str = r#"
  CREATE TRIGGER IF NOT EXISTS body_fts_insert_trigger
  AFTER INSERT ON body
  BEGIN
      INSERT INTO body_fts (metadata_id, text, title, url, last_parsed)
      VALUES (NEW.metadata_id, NEW.text, NEW.title, NEW.url, NEW.last_parsed);
  END;
"#;

pub const TRIGGER_UPDATE_BODY_FTS : &str = r#"
  CREATE TRIGGER IF NOT EXISTS body_fts_update_trigger
  AFTER UPDATE ON body
  BEGIN
      UPDATE body_fts
      SET text = NEW.text
      WHERE metadata_id = NEW.metadata_id;
  END;
"#;

// Don't need this because deleting from body automatically removes from body_fts
pub const TRIGGER_DELETE_BODY_FTS : &str = r#"
  CREATE TRIGGER IF NOT EXISTS body_fts_delete_trigger
  BEFORE DELETE ON body
  BEGIN
      DELETE FROM body_fts WHERE metadata_id = old.metadata_id;
  END;
"#;

////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////// USER PREFS & APP DATA //////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////

// USER_PREFS stores user preferences
pub const USER_PREFS_TABLE_CREATE_STATEMENT : &str = r#"
  CREATE TABLE IF NOT EXISTS "user_preferences" 
  (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    first_launch_done BOOLEAN NOT NULL DEFAULT 0,
    onboarding_done BOOLEAN NOT NULL DEFAULT 0,
    launch_at_startup BOOLEAN NOT NULL DEFAULT 0,
    show_in_dock BOOLEAN NOT NULL DEFAULT 1,
    global_shortcut_enabled BOOLEAN NOT NULL DEFAULT 1,
    global_shortcut TEXT NOT NULL DEFAULT "Alt+Space",
    automatic_background_sync BOOLEAN NOT NULL DEFAULT 1,
    detailed_scan BOOLEAN NOT NULL DEFAULT 1,
    disallowed_paths TEXT NOT NULL DEFAULT ""
  );
"#;

// APP_DATA stores basic app data and file type data
pub const APP_DATA_TABLE_CREATE_STATEMENT : &str = r#"
  CREATE TABLE IF NOT EXISTS "app_data" 
  (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    app_name TEXT NOT NULL DEFAULT "Buzee",
    app_version TEXT NOT NULL DEFAULT "0.1.0",
    app_mode TEXT NOT NULL DEFAULT "window",
    app_theme TEXT NOT NULL DEFAULT "system",
    app_language TEXT NOT NULL DEFAULT "en",
    last_scan_time BIGINT NOT NULL DEFAULT 0,
    scan_running BOOLEAN NOT NULL DEFAULT 0
  );
"#;

// FILE_TYPES stores file types and their categories
pub const FILE_TYPES_TABLE_CREATE_STATEMENT : &str = r#"
  CREATE TABLE IF NOT EXISTS "file_types" 
  (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    added_by_user BOOLEAN NOT NULL DEFAULT 0,
    file_type TEXT NOT NULL DEFAULT "",
    file_type_category TEXT NOT NULL DEFAULT "",
    file_type_allowed BOOLEAN NOT NULL DEFAULT 1
  );
"#;

////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////
//////////////////////////////////////// OTHER DOMAINS /////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////////////////////////////

// domain = "gmail", "outlook", "yahoo", "icloud" etc.
// sender = email address of the sender
// recipient = email address of the recipient(s)
// subject = subject of the email
// sent_at = timestamp when the email was sent
// sent_or_received_at = timestamp when the email was sent/received
// is_read = boolean to indicate if the email is read
// is_starred = boolean to indicate if the email is starred
// is_archived = boolean to indicate if the email is archived
// frecency_rank = float to indicate the frecency of the email
// frecency_last_accessed = timestamp when the email was last accessed using the app
// thread_id = id of the email thread (to group emails in a thread)
// label = label/folder of the email set by the user on the domain
// attachment_count = number of attachments in the email
// comment = user comment added in the app
pub const EMAIL_TABLE_CREATE_STATEMENT : &str = r#"
  CREATE TABLE IF NOT EXISTS "email" 
  (
    "id" integer PRIMARY KEY AUTOINCREMENT NOT NULL, 
    "domain" text NOT NULL,
    "sender" text NOT NULL,
    "recipient" text NOT NULL,
    "subject" text,
    "sent_or_received_at" BIGINT,
    "is_read" BOOLEAN NOT NULL DEFAULT 0,
    "is_starred" BOOLEAN NOT NULL DEFAULT 0,
    "is_archived" BOOLEAN NOT NULL DEFAULT 0,
    "frecency_rank" REAL NOT NULL DEFAULT 0,
    "frecency_last_accessed" BIGINT,
    "thread_id" integer,
    "label" text,
    "attachment_count" integer,
    "comment" text
  );
"#;

// domain = "chrome_bookmarks", "firefox_bookmarks", "pocket", "instapaper", "omnivore" etc.
// title = title of the bookmark/article
// url = url of the bookmark/article
// source = source of the bookmark/article
// saved_at = timestamp when the bookmark/article was saved in the domain
// read_at = timestamp when the bookmark/article was read in the domain
// word_count = word count of the bookmark/article
// is_favorite = boolean to indicate if the bookmark/article is favorite on the domain
// is_archived = boolean to indicate if the bookmark/article is archived on the domain
// is_read = boolean to indicate if the bookmark/article is read on the domain
// tags = tags added to the bookmark/article on the domain
// frecency_rank = float to indicate the frecency of the bookmark/article
// frecency_last_accessed = timestamp when the bookmark/article was last accessed using the app
pub const BOOKMARK_TABLE_CREATE_STATEMENT : &str = r#"
  CREATE TABLE IF NOT EXISTS "bookmark" 
  (
    "id" integer PRIMARY KEY AUTOINCREMENT NOT NULL, 
    "domain" text NOT NULL,
    "title" text NOT NULL,
    "url" text NOT NULL,
    "source" text,
    "saved_at" BIGINT NOT NULL,
    "read_at" BIGINT,
    "word_count" integer,
    "is_favorite" BOOLEAN NOT NULL DEFAULT 0,
    "is_archived" BOOLEAN NOT NULL DEFAULT 0,
    "is_read" BOOLEAN NOT NULL DEFAULT 0,
    "tags" text,
    "frecency_rank" REAL NOT NULL DEFAULT 0,
    "frecency_last_accessed" BIGINT,
    "comment" text
  );
"#;

// domain = "chrome_history", "firefox_history", "safari_history", "edge_history", "newsletters", "rss_feeds", "podcasts",
// url = url of the website
// title = title of the website
// last_visit_time = timestamp when the website was last visited on the domain
// is_bookmarked = boolean to indicate if the website is bookmarked on the domain
// frecency_rank = float to indicate the frecency of the website
// frecency_last_accessed = timestamp when the website was last accessed using the app
pub const WEBSITE_TABLE_CREATE_STATEMENT : &str = r#"
  CREATE TABLE IF NOT EXISTS "website" 
  (
    "id" integer PRIMARY KEY AUTOINCREMENT NOT NULL, 
    "domain" text NOT NULL,
    "url" text NOT NULL,
    "title" text,
    "last_visit_time" BIGINT,
    "is_bookmarked" BOOLEAN NOT NULL DEFAULT 0,
    "frecency_rank" REAL NOT NULL DEFAULT 0,
    "frecency_last_accessed" BIGINT,
    "comment" text
  );
"#;
