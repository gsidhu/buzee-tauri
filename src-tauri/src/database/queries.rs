pub const DOCUMENT_TABLE_CREATE_STATEMENT : &str = r#"
  CREATE TABLE IF NOT EXISTS "document" 
  (
    "id" integer PRIMARY KEY AUTOINCREMENT NOT NULL, 
    "created_at" datetime NOT NULL DEFAULT (datetime('now')),
    "name" text NOT NULL,
    "path" text NOT NULL,
    "size" integer,
    "file_type" varchar NOT NULL,
    "content" text,
    "last_modified" datetime NOT NULL,
    "last_opened" datetime NOT NULL DEFAULT (datetime('now'))
  )
"#;

pub const CREATE_DOCUMENT_FTS : &str = r#"
  CREATE VIRTUAL TABLE IF NOT EXISTS document_fts 
  USING fts5(
    path,
    name,
    content,
    file_type,
    last_modified,
    document_id UNINDEXED,
    content=text,
    tokenize='porter ascii'
  )
"#;

pub const TRIGGER_DOCUMENT_INSERT : &str = r#"
  CREATE TRIGGER IF NOT EXISTS document_insert 
  AFTER INSERT ON document 
  BEGIN INSERT INTO document_fts
  (
    path,
    name,
    content,
    file_type,
    last_modified,
    document_id
  )
  VALUES (
    new.path,
    new.name,
    new.content,
    new.file_type,
    new.last_modified,
    new.id
  ); 
  END;
"#;

pub const TRIGGER_DOCUMENT_UPDATE : &str = r#"
  CREATE TRIGGER IF NOT EXISTS document_update 
  AFTER UPDATE ON document 
  BEGIN 
    DELETE FROM document_fts
    WHERE document_id = old.id;
    
    INSERT INTO document_fts
    (
      path,
      name,
      content,
      file_type,
      last_modified,
      document_id
    )
    VALUES (
      new.path,
      new.name,
      new.content,
      new.file_type,
      new.last_modified,
      new.id
    ); 
  END;
"#;

pub const TRIGGER_DOCUMENT_DELETE : &str = r#"
  CREATE TRIGGER document_delete
  AFTER DELETE ON document
  BEGIN
    DELETE FROM document_fts
    WHERE document_id = old.id;
  END
"#;