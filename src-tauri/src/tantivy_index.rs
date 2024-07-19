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
use std::collections::HashMap;
use std::path::PathBuf;
use crate::custom_types::{Error, TantivyDocumentItem, TantivyDocumentSearchResult, TantivyBookmarkSearchResult};
use std::sync::Mutex;
use tauri::Manager;
use crate::custom_types::TantivyReaderState;

pub fn create_tantivy_schema() -> Schema {
  let mut schema_builder = Schema::builder();

  // common attributes
  // ID here refers to Source_ID in the Metadata Table in the SQLite database
  schema_builder.add_i64_field("id", INDEXED | STORED);
  schema_builder.add_text_field("source_table", STRING);
  schema_builder.add_text_field("source_domain", STRING);
  schema_builder.add_text_field("comment", TEXT);

  // documents attributes
  schema_builder.add_text_field("title", TEXT);
  schema_builder.add_text_field("body", TEXT);
  schema_builder.add_text_field("file_type", STRING);
  schema_builder.add_i64_field("last_modified", INDEXED | STORED);

  // additional browser history attributes
  // URL doubles up as PATH for documents
  schema_builder.add_text_field("url", STRING);

  // additional bookmarks/articles attributes
  schema_builder.add_text_field("tags", TEXT);

  // additional email attributes
  schema_builder.add_text_field("sender", STRING);
  schema_builder.add_text_field("recipient", STRING);
  schema_builder.add_text_field("cc", STRING);
  schema_builder.add_text_field("bcc", STRING);
  schema_builder.add_text_field("subject", TEXT);
  schema_builder.add_text_field("attachments", TEXT);

  schema_builder.build()
}

pub fn get_tantivy_index(schema: Schema) -> tantivy::Result<Index> {
  let index_path = PathBuf::from(norm(format!("{}/{}", get_app_directory(), "buzee_tantivy_index").as_str()));
  let meta_file_path = PathBuf::from(norm(format!("{}/{}/meta.json", get_app_directory(), "buzee_tantivy_index").as_str()));
  if index_path.exists() && meta_file_path.exists() {
    return Index::open_in_dir(&index_path);
  } else {
    return Index::create_in_dir(index_path, schema);
  }
}

pub fn get_reader_for_index(index: &Index) -> tantivy::Result<tantivy::IndexReader> {
  let reader = index
      .reader_builder()
      .reload_policy(ReloadPolicy::OnCommitWithDelay)
      .try_into()?;
  Ok(reader)
}

pub fn _reload_reader(app: &tauri::AppHandle) {
  // Acquire reader from the mutex and reload it
  let state_mutex = app.state::<Mutex<TantivyReaderState>>();
  let mut state = state_mutex.lock().unwrap();
  let reader = &state.reader;
  reader.reload().unwrap();
  state.reader = reader.clone();
}

pub fn acquire_searcher_from_reader(app: &tauri::AppHandle) -> Result<Searcher, Error> {
  // Acquire a searcher from the reader
  let state_mutex = app.state::<Mutex<TantivyReaderState>>();
  let state = state_mutex.lock().unwrap();
  let reader = &state.reader;
  let searcher = reader.searcher();
  Ok(searcher)
}

pub fn add_docs_to_index(files_array: &Vec<TantivyDocumentItem>,) -> tantivy::Result<()> {
  let index = get_tantivy_index(create_tantivy_schema()).unwrap();
  let mut index_writer: IndexWriter = index.writer(150_000_000)?;

  // Get the fields
  let id = index.schema().get_field("id").unwrap();
  let source_table = index.schema().get_field("source_table").unwrap();
  let source_domain = index.schema().get_field("source_domain").unwrap();
  let title = index.schema().get_field("title").unwrap();
  let body = index.schema().get_field("body").unwrap();
  let url = index.schema().get_field("url").unwrap();
  let file_type = index.schema().get_field("file_type").unwrap();
  let last_modified = index.schema().get_field("last_modified").unwrap();
  let comment = index.schema().get_field("comment").unwrap();

  // for each document in the array, add it to the index
  for doc in files_array {
    index_writer.add_document(doc!(
      id => doc.source_id,
      source_table => doc.source_table.as_str(),
      source_domain => doc.source_domain.as_str(),
      title => doc.name.as_str(),
      body => doc.body.as_str(),
      url => doc.url.as_str(),
      file_type => doc.file_type.as_str(),
      last_modified => doc.last_modified,
      comment => doc.comment.as_str(),
    ))?;
  }

  let commit_stamp = index_writer.commit()?;

  if commit_stamp > 0 {
    return Ok(());
  } else {
    return Err(tantivy::TantivyError::SystemError("Failed to commit changes to the index".to_string()));
  }
}

pub fn delete_docs_from_index_with_ids(ids_to_delete: &Vec<i32>) -> tantivy::Result<()> {
  let index = get_tantivy_index(create_tantivy_schema()).unwrap();
  let mut index_writer: IndexWriter = index.writer(150_000_000)?;

  // for each ID in the array, find the document and delete it
  for id in ids_to_delete {
    index_writer.delete_term(Term::from_field_i64(index.schema().get_field("id").unwrap(), i64::from(*id)));
  }

  let commit_stamp = index_writer.commit()?;

  if commit_stamp > 0 {
    return Ok(());
  } else {
    return Err(tantivy::TantivyError::SystemError("Failed to commit changes to the index".to_string()));
  }
}

pub fn delete_all_docs_from_index() -> tantivy::Result<()> {
  println!("WARNING: Deleting all documents from the index");
  let index = get_tantivy_index(create_tantivy_schema()).unwrap();
  let mut index_writer: IndexWriter = index.writer(150_000_000)?;

  let _ = index_writer.delete_all_documents().unwrap();

  let commit_stamp = index_writer.commit()?;

  if commit_stamp > 0 {
    return Ok(());
  } else {
    return Err(tantivy::TantivyError::SystemError("Failed to commit changes to the index".to_string()));
  }
}

pub fn parse_query_and_get_top_docs(index: &Index, searcher: &Searcher, user_query: String, result_limit: i32, result_offset: i32) -> Result<Vec<(f32, DocAddress)>, TantivyError> {
  // Get the fields
  let comment = index.schema().get_field("comment").unwrap();
  let title = index.schema().get_field("title").unwrap();
  let body = index.schema().get_field("body").unwrap();
  let file_type = index.schema().get_field("file_type").unwrap();
  let url = index.schema().get_field("url").unwrap();
  let tags = index.schema().get_field("tags").unwrap();
  let sender = index.schema().get_field("sender").unwrap();
  let recipient = index.schema().get_field("recipient").unwrap();
  let cc = index.schema().get_field("cc").unwrap();
  let bcc = index.schema().get_field("bcc").unwrap();
  let subject = index.schema().get_field("subject").unwrap();
  let attachments = index.schema().get_field("attachments").unwrap();

  // Parse the query
  let mut query_parser = QueryParser::for_index(&index, vec![comment, title, body, file_type, url, tags, sender, recipient, cc, bcc, subject, attachments]);
  query_parser.set_conjunction_by_default();
  let query = query_parser.parse_query(&user_query)?;

  // Search the index
  let top_docs = searcher.search(&query, &TopDocs::with_limit(result_limit.try_into().unwrap()).and_offset(result_offset.try_into().unwrap()))?;

  Ok(top_docs)
}

pub fn return_document_search_results(index: &Index, searcher: &Searcher, top_docs: Vec<(f32, DocAddress)>) -> Result<Vec<TantivyDocumentSearchResult>, TantivyError> {
  // Get the fields
  let id = index.schema().get_field("id").unwrap();
  let last_modified = index.schema().get_field("last_modified").unwrap();

  // Retrieve the search results
  let mut search_results = Vec::new();
  for (_score, doc_address) in top_docs {
    let retrieved_doc: TantivyDocument = searcher.doc(doc_address).ok().unwrap();
    // println!("Retrieved doc: {:?}", retrieved_doc.to_json(&index.schema()));
    let result: TantivyDocumentSearchResult = {
      TantivyDocumentSearchResult {
        id: retrieved_doc.get_first(id).and_then(|value| value.as_i64()).unwrap_or_else(|| {return 0_i64 ;}),
        last_modified: retrieved_doc.get_first(last_modified).and_then(|value| value.as_i64()).unwrap_or_else(|| {return 0_i64 ;}),
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
  let frecency_rank = index.schema().get_field("frecency_rank").unwrap();
  let frecency_last_accessed = index.schema().get_field("frecency_last_accessed").unwrap();

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
        frecency_rank: retrieved_doc.get_first(frecency_rank).and_then(|value| value.as_f64()).or(None),
        frecency_last_accessed: retrieved_doc.get_first(frecency_last_accessed).and_then(|value| value.as_i64()).or(None),
      }
    };

    search_results.push(result);
  }

  Ok(search_results)
}

pub fn _get_all_unique_ids_from_index(searcher: &Searcher) -> Vec<i64> {
  let index = get_tantivy_index(create_tantivy_schema()).unwrap();
  let id = index.schema().get_field("id").unwrap();

  // find all documents in the index using tantivy::query::AllQuery
  let query = QueryParser::for_index(&index, vec![id]).parse_query("*").unwrap();
  let top_docs = searcher.search(&query, &TopDocs::with_limit(100_000_000)).unwrap();

  let all_ids: Vec<i64> = top_docs.iter().map(|(_score, doc_address)| {
    let retrieved_doc: TantivyDocument = searcher.doc(*doc_address).ok().unwrap();
    retrieved_doc.get_first(id).and_then(|value| value.as_i64()).unwrap_or_else(|| {return 0_i64 ;})
  }).collect();

  let unique_ids = all_ids.into_iter().collect::<std::collections::HashSet<i64>>().into_iter().collect();

  unique_ids
}

pub fn _get_body_values_from_id(searcher: &Searcher, given_id: i64) -> Vec<String> {
  let index = get_tantivy_index(create_tantivy_schema()).unwrap();
  let id = index.schema().get_field("id").unwrap();
  let body = index.schema().get_field("body").unwrap();
  let chunk_id = index.schema().get_field("chunk_id").unwrap();

  // find the document in the index where the ID matches the given ID
  let query = QueryParser::for_index(&index, vec![id]).parse_query(format!("id:{}", given_id).as_str()).unwrap();
  let top_docs = searcher.search(&query, &TopDocs::with_limit(10000)).unwrap();

  // create a hashmap with chunk_id as key and body_value as value
  let mut body_values = HashMap::new();
  for (_score, doc_address) in top_docs {
    let retrieved_doc: TantivyDocument = searcher.doc(doc_address).ok().unwrap();
    let body_value = retrieved_doc.get_first(body).and_then(|value| value.as_str()).unwrap_or_else(|| {return "" ;}).to_string();
    let chunk_id_value = retrieved_doc.get_first(chunk_id).and_then(|value| value.as_i64()).unwrap_or_else(|| {return 0_i64 ;});
    body_values.insert(chunk_id_value, body_value);
  }

  // sort the hashmap by chunk_id and return the values
  let mut sorted_body_values = body_values.into_iter().collect::<Vec<_>>();
  sorted_body_values.sort_by(|a, b| a.0.cmp(&b.0));
  let body_values = sorted_body_values.into_iter().map(|(_k, v)| v).collect::<Vec<String>>();

  body_values
}

pub fn internal_test_create_csv_dump_from_index(searcher: &Searcher) {
  println!("Creating a CSV dump from the index");
  let index = get_tantivy_index(create_tantivy_schema()).unwrap();

  let id = index.schema().get_field("id").unwrap();
  let url = index.schema().get_field("url").unwrap();
  let last_parsed = index.schema().get_field("last_parsed").unwrap();
  let chunk_id = index.schema().get_field("chunk_id").unwrap();

  // find all documents in the index using tantivy::query::AllQuery
  let query = QueryParser::for_index(&index, vec![url, last_parsed, chunk_id]).parse_query("*").unwrap();
  let top_docs = searcher.search(&query, &TopDocs::with_limit(100_000)).unwrap();

  println!("Found {} items in the index", top_docs.len());

  // create a CSV file with the results
  let mut wtr = csv::Writer::from_path("/Users/thatgurjot/Documents/buzee-tauri/search_results.csv").unwrap();
  wtr.write_record(&["id", "last_parsed", "chunk_id", "url"]).unwrap();
  for (_score, doc_address) in top_docs {
    let retrieved_doc: TantivyDocument = searcher.doc(doc_address).ok().unwrap();
    let id_value = retrieved_doc.get_first(id).and_then(|value| value.as_i64()).unwrap_or_else(|| {return 0_i64 ;});
    let url_value = retrieved_doc.get_first(url).and_then(|value| value.as_str()).unwrap_or_else(|| {return "null" ;}).to_string();
    let last_parsed_value = retrieved_doc.get_first(last_parsed).and_then(|value| value.as_i64()).unwrap_or_else(|| {return 0_i64 ;});
    let chunk_id_value = retrieved_doc.get_first(chunk_id).and_then(|value| value.as_i64()).unwrap_or_else(|| {return 0_i64 ;});
    wtr.write_record(&[id_value.to_string(), last_parsed_value.to_string(), chunk_id_value.to_string(), url_value]).unwrap();
  }
  wtr.flush().unwrap();

  println!("CSV dump created successfully");
}
