// Note: Even if some of the returned results from the index have some fields missing, the retrieval functions accommodate it with default null values. The custom return types use Option<T> to handle this. This prevents application panic.
// The three required fields are: id, source_table and source_domain. The rest of the fields are optional.
// Best practice is to define the `source_table` in the search query to filter out any unwanted results. 
// For e.g. if you are searching for bookmarks, then the search query should start with `source_table:bookmarks`.

use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::{schema::*, DocAddress};
use tantivy::{doc, Index, IndexWriter, ReloadPolicy, Searcher, TantivyError};
use crate::housekeeping::get_app_directory;
use crate::utils::norm;
use std::path::PathBuf;
use crate::custom_types::{Error, TantivyDocumentSearchResult, TantivyBookmarkSearchResult};
use std::sync::Mutex;
use tauri::Manager;
use crate::custom_types::TantivyReaderState;

pub fn create_tantivy_schema() -> Schema {
  let mut schema_builder = Schema::builder();

  // common attributes
  schema_builder.add_i64_field("id", STORED);
  schema_builder.add_text_field("source_table", TEXT | STORED);
  schema_builder.add_text_field("source_domain", TEXT | STORED);
  schema_builder.add_bool_field("is_pinned", STORED);
  schema_builder.add_text_field("comment", TEXT | STORED);

  // documents attributes
  schema_builder.add_i64_field("created_at", STORED);
  schema_builder.add_text_field("title", TEXT | STORED);
  schema_builder.add_text_field("body", TEXT | STORED);
  schema_builder.add_text_field("path", TEXT | STORED);
  schema_builder.add_f64_field("size", STORED);
  schema_builder.add_text_field("file_type", TEXT | STORED);
  schema_builder.add_i64_field("last_modified", STORED);
  schema_builder.add_i64_field("last_opened", STORED);
  schema_builder.add_i64_field("last_synced", STORED);
  schema_builder.add_i64_field("last_parsed", STORED);

  // additional browser history attributes
  schema_builder.add_text_field("url", TEXT | STORED);
  schema_builder.add_i64_field("last_visited", STORED);

  // additional bookmarks/articles attributes
  schema_builder.add_i64_field("saved_at", STORED);
  schema_builder.add_i64_field("word_count", STORED);
  schema_builder.add_bool_field("is_favorite", STORED);
  schema_builder.add_bool_field("is_archived", STORED);
  schema_builder.add_bool_field("is_read", STORED);
  schema_builder.add_text_field("tags", TEXT | STORED);

  // additional email attributes
  schema_builder.add_text_field("sender", TEXT | STORED);
  schema_builder.add_text_field("recipient", TEXT | STORED);
  schema_builder.add_text_field("cc", TEXT | STORED);
  schema_builder.add_text_field("bcc", TEXT | STORED);
  schema_builder.add_text_field("subject", TEXT | STORED);
  schema_builder.add_text_field("attachments", TEXT | STORED);

  schema_builder.build()
}

pub fn get_tantivy_index(schema: Schema) -> tantivy::Result<Index> {
  let index_path = PathBuf::from(norm(format!("{}/{}", get_app_directory(), "tantivy_index").as_str()));
  let meta_file_path = PathBuf::from(norm(format!("{}/{}/meta.json", get_app_directory(), "tantivy_index").as_str()));
  if index_path.exists() && meta_file_path.exists() {
    return Index::open_in_dir(&index_path);
  } else {
    return Index::create_in_dir(index_path, schema);
  }
}

pub fn add_docs_to_index(index: &Index, ) -> tantivy::Result<()> {
  let mut index_writer: IndexWriter = index.writer(50_000_000)?;

  // Get the fields
  let id = index.schema().get_field("id").unwrap();
  let source_table = index.schema().get_field("source_table").unwrap();
  let source_domain = index.schema().get_field("source_domain").unwrap();
  let is_pinned = index.schema().get_field("is_pinned").unwrap();
  let comment = index.schema().get_field("comment").unwrap();

  let created_at = index.schema().get_field("created_at").unwrap();
  let title = index.schema().get_field("title").unwrap();
  let body = index.schema().get_field("body").unwrap();
  let path = index.schema().get_field("path").unwrap();
  let size = index.schema().get_field("size").unwrap();
  let file_type = index.schema().get_field("file_type").unwrap();
  let last_modified = index.schema().get_field("last_modified").unwrap();
  let last_opened = index.schema().get_field("last_opened").unwrap();
  let last_synced = index.schema().get_field("last_synced").unwrap();
  let last_parsed = index.schema().get_field("last_parsed").unwrap();

  let url = index.schema().get_field("url").unwrap();
  let last_visited = index.schema().get_field("last_visited").unwrap();

  let saved_at = index.schema().get_field("saved_at").unwrap();
  let word_count = index.schema().get_field("word_count").unwrap();
  let is_favorite = index.schema().get_field("is_favorite").unwrap();
  let is_archived = index.schema().get_field("is_archived").unwrap();
  let is_read = index.schema().get_field("is_read").unwrap();
  let tags = index.schema().get_field("tags").unwrap();

  let sender = index.schema().get_field("sender").unwrap();
  let recipient = index.schema().get_field("recipient").unwrap();
  let cc = index.schema().get_field("cc").unwrap();
  let bcc = index.schema().get_field("bcc").unwrap();
  let subject = index.schema().get_field("subject").unwrap();
  let attachments = index.schema().get_field("attachments").unwrap();

  // Sample Document - 1
  index_writer.add_document(doc!(
    id => 0_i64,
    source_table => "files",
    source_domain => "document",
    is_pinned => true,
    comment => "This is a great book",
    created_at => 1614556800_i64,
    title => "Of Mice and Men",
    body => "A few miles south of Soledad, the Buzo Salinas River drops in close to the hillside \
            bank and runs deep and green. The water is warm too, for it has slipped twinkling \
            over the yellow sands in the sunlight before reaching the narrow pool. On one \
            side of the river the golden foothill slopes curve up to the strong and rocky \
            Gabilan Mountains, but on the valley side the water is lined with trees—willows \
            fresh and green with every spring, carrying in their lower leaf junctures the \
            debris of the winter’s flooding; and sycamores with mottled, white, recumbent \
            limbs and branches that arch over the pool",
    path => "/home/user/Documents/",
    size => 1024.0,
    file_type => "pdf",
    last_modified => 1614556800_i64,
    last_opened => 1614556800_i64,
    last_synced => 1614556800_i64,
    last_parsed => 1614556800_i64,
  ))?;

  // Sample Document - 2
  index_writer.add_document(doc!(
    id => 1_i64,
    source_table => "files",
    source_domain => "document",
    is_pinned => false,
    comment => "This is a great book",
    created_at => 1614556800_i64,
    title => "Arcadia",
    body => "Arcadia is a play by Tom Stoppard concerning the relationship between past and present, \
            order and disorder, certainty and uncertainty. It has been praised by many critics as \
            the finest play from one of the most significant contemporary playwrights in the English \
            language. In 2006, the Royal Institution of Great Britain named it one of the best science-related \
            works ever written.",
    path => "/home/user/Documents/",
    size => 1024.0,
  ))?;

  // Sample Browser History
  index_writer.add_document(doc!(
    id => 0_i64,
    source_table => "web_history",
    source_domain => "chrome",
    is_pinned => true,
    comment => "This is a great book",
    title => "Of Mice and Men",
    body => "Site description goes here... romeo",
    url => "https://en.wikipedia.org/",
    last_visited => 1614556800_i64,
  ))?;

  // Sample Bookmark/Article
  index_writer.add_document(doc!(
    id => 0_i64,
    source_table => "bookmarks",
    source_domain => "instapaper",
    title => "Romeo and Juliet",
    url => "https://en.wikipedia.org/Romeo_and_Juliet",
    body => "Romeo and Juliet is a tragedy written by William Shakespeare early in his career about two young star-crossed lovers whose deaths ultimately reconcile their feuding families. It was among Shakespeare's most popular plays during his lifetime and along with Hamlet, is one of his most frequently performed plays. Today, the title characters are regarded as archetypal young lovers.",
    saved_at => 1614556800_i64,
    last_visited => 1614556800_i64,
    word_count => 1024_i64,
    is_favorite => true,
    is_archived => false,
    is_read => false,
    tags => "love, tragedy",
  ))?;

  // Sample Email
  index_writer.add_document(doc!(
    id => 0_i64,
    source_table => "email",
    source_domain => "gmail",
    is_pinned => true,
    comment => "This is the best email",
    sender => "buzo@buzo.com",
    recipient => "goju@buzo.com",
    cc => "pogo@buzo.com",
    bcc => "",
    subject => "Hello",
    body => "Bow bow this is a great email",
    attachments => "file1.pdf, file2.docx",
    tags => "important",
  ))?;

  index_writer.commit()?;

  Ok(())

}

pub fn get_reader_for_index(index: &Index) -> tantivy::Result<tantivy::IndexReader> {
  let reader = index
      .reader_builder()
      .reload_policy(ReloadPolicy::OnCommitWithDelay)
      .try_into()?;
  Ok(reader)
}

pub fn acquire_searcher_from_reader(app: &tauri::AppHandle) -> Result<Searcher, Error> {
  // Acquire a searcher from the reader
  let state_mutex = app.state::<Mutex<TantivyReaderState>>();
  let state = state_mutex.lock().unwrap();
  let reader = &state.reader;
  let searcher = reader.searcher();

  Ok(searcher)
}

pub fn parse_query_and_get_top_docs(index: &Index, searcher: &Searcher, user_query: String, result_limit: i32) -> Result<Vec<(f32, DocAddress)>, TantivyError> {
  // Get the fields
  let comment = index.schema().get_field("comment").unwrap();
  let title = index.schema().get_field("title").unwrap();
  let body = index.schema().get_field("body").unwrap();
  let path = index.schema().get_field("path").unwrap();
  let file_type = index.schema().get_field("file_type").unwrap();
  let url = index.schema().get_field("url").unwrap();

  let sender = index.schema().get_field("sender").unwrap();
  let recipient = index.schema().get_field("recipient").unwrap();
  let cc = index.schema().get_field("cc").unwrap();
  let bcc = index.schema().get_field("bcc").unwrap();
  let subject = index.schema().get_field("subject").unwrap();
  let attachments = index.schema().get_field("attachments").unwrap();

  let tags = index.schema().get_field("tags").unwrap();

  // Parse the query
  let query_parser = QueryParser::for_index(&index, vec![comment, title, body, path, file_type, url, tags, sender, recipient, cc, bcc, subject, attachments]);
  let query = query_parser.parse_query(&user_query)?;

  // Search the index
  let top_docs = searcher.search(&query, &TopDocs::with_limit(result_limit.try_into().unwrap()))?;

  Ok(top_docs)
}

pub fn return_document_search_results(index: &Index, searcher: &Searcher, top_docs: Vec<(f32, DocAddress)>) -> Result<Vec<TantivyDocumentSearchResult>, TantivyError> {
  // Get the fields
  let id = index.schema().get_field("id").unwrap();
  let source_table = index.schema().get_field("source_table").unwrap();
  let source_domain = index.schema().get_field("source_domain").unwrap();
  let is_pinned = index.schema().get_field("is_pinned").unwrap();
  let comment = index.schema().get_field("comment").unwrap();

  let created_at = index.schema().get_field("created_at").unwrap();
  let title = index.schema().get_field("title").unwrap();
  let body = index.schema().get_field("body").unwrap();
  let path = index.schema().get_field("path").unwrap();
  let size = index.schema().get_field("size").unwrap();
  let file_type = index.schema().get_field("file_type").unwrap();
  let last_modified = index.schema().get_field("last_modified").unwrap();
  let last_opened = index.schema().get_field("last_opened").unwrap();
  let last_synced = index.schema().get_field("last_synced").unwrap();
  let last_parsed = index.schema().get_field("last_parsed").unwrap();

  // Retrieve the search results
  let mut search_results = Vec::new();
  for (_score, doc_address) in top_docs {
    let retrieved_doc: TantivyDocument = searcher.doc(doc_address).ok().unwrap();
    println!("Retrieved doc: {:?}", retrieved_doc.to_json(&index.schema()));
    let result: TantivyDocumentSearchResult = {
      TantivyDocumentSearchResult {
        id: retrieved_doc.get_first(id).and_then(|value| value.as_i64()).unwrap_or_else(|| {return 0_i64 ;}),
        source_table: retrieved_doc.get_first(source_table).and_then(|value| value.as_str()).unwrap_or_else(|| {return "null" ;}).to_string(),
        source_domain: retrieved_doc.get_first(source_domain).and_then(|value| value.as_str()).unwrap_or_else(|| {return "null" ;}).to_string(),
        is_pinned: retrieved_doc.get_first(is_pinned).and_then(|value| value.as_bool()).or(None),
        comment: retrieved_doc.get_first(comment).and_then(|value| value.as_str().map(|s| s.to_string())).or(None),
        created_at: retrieved_doc.get_first(created_at).and_then(|value| value.as_i64()).or(None),
        title: retrieved_doc.get_first(title).and_then(|value| value.as_str().map(|s| s.to_string())).or(None),
        body: retrieved_doc.get_first(body).and_then(|value| value.as_str().map(|s| s.to_string())).or(None),
        path: retrieved_doc.get_first(path).and_then(|value| value.as_str().map(|s| s.to_string())).or(None),
        size: retrieved_doc.get_first(size).and_then(|value| value.as_f64()).or(None),
        file_type: retrieved_doc.get_first(file_type).and_then(|value| value.as_str().map(|s| s.to_string())).or(None),
        last_modified: retrieved_doc.get_first(last_modified).and_then(|value| value.as_i64()).or(None),
        last_opened: retrieved_doc.get_first(last_opened).and_then(|value| value.as_i64()).or(None),
        last_synced: retrieved_doc.get_first(last_synced).and_then(|value| value.as_i64()).or(None),
        last_parsed: retrieved_doc.get_first(last_parsed).and_then(|value| value.as_i64()).or(None),
        
      }
    };
    search_results.push(result);
  }

  Ok(search_results)
}

pub fn return_bookmark_search_results(index: &Index, searcher: &Searcher, top_docs: Vec<(f32, DocAddress)>) -> Result<Vec<TantivyBookmarkSearchResult>, TantivyError> {
  // Get the fields
  let id = index.schema().get_field("id").unwrap();
  let source_table = index.schema().get_field("source_table").unwrap();
  let source_domain = index.schema().get_field("source_domain").unwrap();
  let is_pinned = index.schema().get_field("is_pinned").unwrap();
  let comment = index.schema().get_field("comment").unwrap();

  let title = index.schema().get_field("title").unwrap();
  let body = index.schema().get_field("body").unwrap();
  let url = index.schema().get_field("url").unwrap();
  let saved_at = index.schema().get_field("saved_at").unwrap();
  let last_opened = index.schema().get_field("last_opened").unwrap();
  let word_count = index.schema().get_field("word_count").unwrap();
  let is_favorite = index.schema().get_field("is_favorite").unwrap();
  let is_archived = index.schema().get_field("is_archived").unwrap();
  let is_read = index.schema().get_field("is_read").unwrap();
  let tags = index.schema().get_field("tags").unwrap();

  // Retrieve the search results
  let mut search_results = Vec::new();
  for (_score, doc_address) in top_docs {
    let retrieved_doc: TantivyDocument = searcher.doc(doc_address).ok().unwrap();
    let result: TantivyBookmarkSearchResult = {
      TantivyBookmarkSearchResult {
        id: retrieved_doc.get_first(id).and_then(|value| value.as_i64()).unwrap_or_else(|| {return 0_i64 ;}),
        source_table: retrieved_doc.get_first(source_table).and_then(|value| value.as_str()).unwrap_or_else(|| {return "null" ;}).to_string(),
        source_domain: retrieved_doc.get_first(source_domain).and_then(|value| value.as_str()).unwrap_or_else(|| {return "null" ;}).to_string(),
        is_pinned: retrieved_doc.get_first(is_pinned).and_then(|value| value.as_bool()).or(None),
        comment: retrieved_doc.get_first(comment).and_then(|value| value.as_str().map(|s| s.to_string())).or(None),
        title: retrieved_doc.get_first(title).and_then(|value| value.as_str().map(|s| s.to_string())).or(None),
        body: retrieved_doc.get_first(body).and_then(|value| value.as_str().map(|s| s.to_string())).or(None),
        url: retrieved_doc.get_first(url).and_then(|value| value.as_str().map(|s| s.to_string())).or(None),
        saved_at: retrieved_doc.get_first(saved_at).and_then(|value| value.as_i64()).or(None),
        last_opened: retrieved_doc.get_first(last_opened).and_then(|value| value.as_i64()).or(None),
        word_count: retrieved_doc.get_first(word_count).and_then(|value| value.as_i64()).or(None),
        is_favorite: retrieved_doc.get_first(is_favorite).and_then(|value| value.as_bool()).or(None),
        is_archived: retrieved_doc.get_first(is_archived).and_then(|value| value.as_bool()).or(None),
        is_read: retrieved_doc.get_first(is_read).and_then(|value| value.as_bool()).or(None),
        tags: retrieved_doc.get_first(tags).and_then(|value| value.as_str().map(|s| s.to_string())).or(None),
      }
    };

    search_results.push(result);
  }

  Ok(search_results)
}

pub fn create_tantivy_basic_example() -> tantivy::Result<()> {
  println!("--- Creating a Tantivy index with a basic example");
  let schema = create_tantivy_schema();
  let index = get_tantivy_index(schema.clone())?;
  add_docs_to_index(&index)?;
  Ok(())
}