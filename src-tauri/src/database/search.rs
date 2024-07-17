use crate::custom_types::{Error, DBStat, DateLimit, QuerySegments};
use crate::database::establish_connection;
use crate::database::models::{DocumentSearchResult, MetadataFTSSearchResult};
use crate::indexing::all_allowed_filetypes;
use crate::tantivy_index::{acquire_searcher_from_reader, create_tantivy_schema, get_tantivy_index, parse_query_and_get_top_docs, return_document_search_results};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};
use diesel::r2d2::{PooledConnection, ConnectionManager};
use serde_json;
use super::schema::{body, document};
use tantivy::{Searcher, Index};

fn parse_stringified_query_segments(json_string: &str) -> QuerySegments {
    let parsed_json = serde_json::from_str(json_string);
    // convert to QuerySegments
    let query_segments: QuerySegments = match parsed_json {
        Ok(value) => value,
        Err(_) => QuerySegments {
            quoted_segments: Vec::new(),
            greedy_segments: Vec::new(),
            not_segments: Vec::new(),
        },
    };
    query_segments
}

fn create_match_statement(query_segments: &QuerySegments) -> String {
    let mut match_string: String = String::new();

    // If there are quoted segments, join them with double quotes
    if query_segments.quoted_segments.len() > 0 {
        match_string = format!("{}",
        // query_segments.quoted_segments.join("\" \"")
        query_segments.quoted_segments
            .iter()
            .map(|segment| {
                // if segment contains ^^, it contains a punctuation mark (using code from frontend because regex caused problems in rust)
                if segment.contains("^^") {
                    // Remove ^^ from the segment and
                    // Add double quotes around the segment
                    format!("\"{}\"", segment.replace("^^", ""))
                } else {
                    // Add double quotes around the segment
                    format!("\"{}\"", segment)
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
        );
    }
    // If there are greedy segments, join them with an asterisk space
    if query_segments.greedy_segments.len() > 0 {
        match_string = format!(
            "{} {}*",
            match_string,
            query_segments.greedy_segments.join("* ")
        );
    }
    // If there are NOT segments, join them with `NOT` and OR
    if query_segments.not_segments.len() > 0 {
        match_string = format!(
            "{} NOT ({})",
            match_string,
            // join with `* OR` and remove the trailing `OR`
            query_segments.not_segments
              .iter()
              .map(|segment| {
                  // if segment starts with ^^, it contains a punctuation mark
                  // or if it has a space in it, it is a phrase
                  if segment.starts_with("^^") || segment.contains(" ") {
                      // Remove ^^ from the segment and
                      // Add double quotes around the segment
                      format!("\"{}\"", segment.replace("^^", ""))
                  } else {
                      // Add * to the end of the segment
                      format!("{}*", segment)
                  }
              })
              .collect::<Vec<String>>()
              .join(" OR ")
        );
    }
    // remove the trailing `OR )`
    match_string = match_string.trim().to_string();
    match_string
}

fn create_tantivy_query_statement(query_segments: &QuerySegments, file_type_string: String) -> String {
    let mut tantivy_query_string: String = String::new();

    // If there are quoted segments, join them with double quotes
    if query_segments.quoted_segments.len() > 0 {
      tantivy_query_string = format!("{}",
        query_segments.quoted_segments
            .iter()
            .map(|segment| {
                // if segment contains ^^, it contains a punctuation mark (using code from frontend because regex caused problems in rust)
                if segment.contains("^^") {
                    // Remove ^^ from the segment and
                    // Add double quotes around the segment
                    format!("\"{}\"", segment.replace("^^", ""))
                } else {
                    // Add double quotes around the segment
                    format!("\"{}\"", segment)
                }
            })
            .collect::<Vec<String>>()
            .join(" ")
        );
    }
    // If there are greedy segments, join them with an asterisk space
    if query_segments.greedy_segments.len() > 0 {
      tantivy_query_string = format!(
            "{} {}*",
            tantivy_query_string,
            query_segments.greedy_segments.join("* ")
        );
    }
    // If there are NOT segments, place a - in front of each segment
    if query_segments.not_segments.len() > 0 {
      tantivy_query_string = format!(
          "{} -{}",
          tantivy_query_string,
          query_segments.not_segments.iter().map(|segment| {
            // if segment contains a space, wrap it in double quotes
            if segment.contains(" ") {
              format!("\"{}\"", segment)
            } else if segment.contains("^^") {
              // Remove ^^ from the segment and
              // Add double quotes around the segment
              format!("\"{}\"", segment.replace("^^", ""))
            } else {
              segment.to_string()
            }
          })
          .collect::<Vec<String>>()
          .join(" -")
        );
    }
    // remove the trailing `OR )`
    tantivy_query_string = tantivy_query_string.trim().to_string();

    if file_type_string.len() == 0 || tantivy_query_string.len() == 0 {
      return tantivy_query_string;
    } else {
      // add file_type to query
      if !file_type_string.contains(",") {
        tantivy_query_string = format!("{} AND file_type:{}", tantivy_query_string, file_type_string);
      } else {
        let file_type_query_string = file_type_string.replace(",", " OR file_type:");
        tantivy_query_string = format!("{} AND (file_type:{})", tantivy_query_string, file_type_query_string);
      }
    }

    tantivy_query_string
}

// Return documents from the metadata_fts index that match the given search query (name and type)
// bm25(document_fts, 10) is the ranking function which gives 10x weight to the file name (first column)
pub fn search_fts_index(
    query: String,
    page: i32,
    limit: i32,
    file_type: Option<String>,
    date_limit: Option<DateLimit>,
    mut conn: PooledConnection<ConnectionManager<SqliteConnection>>,
    app: &tauri::AppHandle
) -> Result<Vec<DocumentSearchResult>, diesel::result::Error> {
    println!(
        "search_fts_index: query: {}, page: {}, limit: {}, file_type: {:?}, date_limit: {:?}",
        query, page, limit, file_type, date_limit
    );

    let query_segments: QuerySegments = parse_stringified_query_segments(&query);
    println!("query_segments: {:?}", query_segments);

    let file_type_clone = file_type.clone();
    let file_type_clone_two = file_type.clone();
    // Add file type(s)
    let where_file_type = if let Some(file_type) = file_type {
      if !file_type.contains(",") {
        format!(r#"file_type IN ('{}')"#, file_type)
      } else {
        format!(
            r#"file_type IN ('{}')"#,
            file_type.replace(",", "','").replace("' ", "'")
        )
      }
    } else {
      "".to_string()
    };
    // Add date limit(s)
    let where_date_limit: String = if let Some(date_limit) = date_limit {
      format!(
        r#"{} last_modified >= '{}' AND last_modified <= '{}'"#,
        if !where_file_type.is_empty() {
          "AND"
        } else {
          ""
        },
        date_limit.start,
        date_limit.end
      )
    } else {
        "".to_string()
    };

    let mut search_results: Vec<DocumentSearchResult> = Vec::new();
    // if there is only a NOT query, pass it to `handle_special_case` function
    if query_segments.quoted_segments.is_empty() && query_segments.greedy_segments.is_empty() && !query_segments.not_segments.is_empty() {
      search_results = handle_special_case(query, page, limit, file_type_clone, conn).unwrap();
    }
    // otherwise run the body and metadata fts queries as usual
    else {
      let match_string = create_match_statement(&query_segments);
      println!("match_string: {}", match_string);
      let tantivy_string = create_tantivy_query_statement(&query_segments, file_type_clone_two.unwrap_or("".to_string()));
      println!("tantivy_string: {}", tantivy_string);

      let tantivy_index = get_tantivy_index(create_tantivy_schema()).unwrap();
      let searcher = acquire_searcher_from_reader(&app).unwrap();
      let new_conn = establish_connection(&app);
      let tantivy_search_results = get_search_results_from_tantivy_index(&tantivy_string, limit, page, &searcher, &tantivy_index, new_conn).unwrap_or(Vec::new());

      // let tantivy_search_results = Vec::new();
      println!("got {} results from tantivy index", tantivy_search_results.len());

      let metadata_fts_query = create_metadata_fts_query(&where_file_type, &where_date_limit, &match_string, limit, page);
      let metadata_search_results: Vec<DocumentSearchResult> = diesel::sql_query(metadata_fts_query).load::<DocumentSearchResult>(&mut conn).unwrap_or(Vec::new());
      println!("got {} results from metadata_fts", metadata_search_results.len());
      // combine the results from body_fts and metadata_fts
      // let metadata_search_results = Vec::new();
      for result in metadata_search_results.iter().chain(tantivy_search_results.iter()) {
        search_results.push(result.clone());
      }
    }
    // and order them by last_modified
    // TODO: change this to frequency rank
    search_results.sort_by(|a, b| b.last_modified.cmp(&a.last_modified));
    // remove duplicates by checking if the id is the same
    search_results.dedup_by(|a, b| a.id == b.id);

    Ok(search_results)
}

fn get_search_results_from_tantivy_index(query: &String, limit: i32, page: i32, searcher: &Searcher, tantivy_index: &Index, mut conn:  PooledConnection<ConnectionManager<SqliteConnection>>,) -> Result<Vec<DocumentSearchResult>, Error> {
  let top_docs = parse_query_and_get_top_docs(&tantivy_index, &searcher, query.to_string(), limit, page*limit).unwrap_or(Vec::new());
  if top_docs.len() > 0 {
    let search_results = return_document_search_results(&tantivy_index, &searcher, top_docs).unwrap_or(vec![]);
    let document_ids: Vec<i32> = search_results.iter().map(|result| result.id as i32).collect();

    let search_results_to_return = document::table
      .filter(document::id.eq_any(document_ids))
      .load::<DocumentSearchResult>(&mut conn)
      .unwrap_or(Vec::new());

    Ok(search_results_to_return)
  } else {
    Ok(Vec::new())
  }
}

fn _create_body_fts_query(
    where_file_type: &String,
    where_date_limit: &String,
    match_string: &String,
    limit: i32,
    page: i32,
) -> String {
    // Give 100x weight to the title/name column (4th) in metadata_fts and 2x weight to the url/path column (5th)
    let inner_query = format!(
        r#" 
            SELECT d.id, d.source_domain, d.created_at,
                d.name, d.path, d.size, d.file_type,
                d.last_modified, d.last_opened, d.last_synced, d.last_parsed,
                d.is_pinned, d.frecency_rank, d.frecency_last_accessed, d.comment
            FROM (
                SELECT DISTINCT metadata_id FROM body_fts
                {match_clause}
            ) b
            JOIN metadata m ON b.metadata_id = m.id
            JOIN document d ON m.source_id = d.id
            {inner_where} {where_file_type} {where_date_limit}
            LIMIT {limit} OFFSET {offset}
        "#,
        match_clause = if !match_string.is_empty() {
            format!(
                "WHERE body_fts MATCH '{}' ORDER BY bm25(body_fts, 1,1,100,2)",
                match_string
            )
        } else {
            "".to_string()
        },
        inner_where = if !where_file_type.is_empty() || !where_date_limit.is_empty() {
            "WHERE".to_string()
        } else {
            "".to_string()
        },
        where_file_type = if !where_file_type.is_empty() {
            where_file_type.clone()
        } else {
            "".to_string()
        },
        where_date_limit = if !where_date_limit.is_empty() {
            // replace `last_modified` with `document.last_modified` in where_date_limit
            where_date_limit.clone().replace("last_modified", "d.last_modified")
        } else {
            "".to_string()
        },
        limit = limit*2,
        offset = page * limit*2
    );

    println!("body_inner_query: {}", inner_query);
    inner_query
}

fn create_metadata_fts_query(
    where_file_type: &String,
    where_date_limit: &String,
    match_string: &String,
    limit: i32,
    page: i32,
) -> String {
    // Give 5x weight to the title column (4th) in metadata_fts
    let inner_query = format!(
        r#"
          SELECT m.source_domain, m.source_id as id, m.title as name, m.url as path, m.created_at, m.frecency_rank, m.frecency_last_accessed, d.file_type, d.size, d.is_pinned, d.comment, d.last_opened, d.last_synced, d.last_modified, d.last_parsed
          FROM metadata_fts m
          JOIN (
              SELECT id, file_type, size, is_pinned, comment, last_opened, last_synced, last_modified, last_parsed
              FROM document
              {inner_where} {where_file_type} {where_date_limit}
          ) d ON m.source_id = d.id
          {match_clause}
          LIMIT {limit} OFFSET {offset}
        "#,
        inner_where = if !where_file_type.is_empty() || !where_date_limit.is_empty() {
            "WHERE".to_string()
        } else {
            "".to_string()
        },
        match_clause = if !match_string.is_empty() {
            format!(
                "WHERE metadata_fts MATCH '{}' ORDER BY bm25(metadata_fts, 1,1,1,1,100,2)",
                match_string
            )
        } else {
            "".to_string()
        },
        where_file_type = if !where_file_type.is_empty() {
            where_file_type.clone()
        } else {
            "".to_string()
        },
        where_date_limit = if !where_date_limit.is_empty() {
            where_date_limit.clone()
        } else {
            "".to_string()
        },
        limit = limit*2,
        offset = page * limit*2
    );

    println!("metadata_inner_query: {}", inner_query);
    inner_query
}

// Get recently opened documents
pub fn get_recently_opened_docs(
    page: i32,
    limit: i32,
    file_type: Option<String>,
    mut conn: PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<Vec<DocumentSearchResult>, diesel::result::Error> {
    // Add file type(s)
    let where_file_type = if let Some(file_type) = file_type {
        if !file_type.contains(",") {
            format!(r#" WHERE file_type IN ('{}')"#, file_type)
        } else {
            format!(
                r#" WHERE file_type IN ('{}')"#,
                file_type.replace(",", "','").replace("' ", "'")
            )
        }
    } else {
        "".to_string()
    };

    let inner_query = format!(
        r#"
        SELECT m.source_domain, m.source_id as id, m.title as name, m.url as path, m.created_at, m.last_modified, m.frecency_rank, m.frecency_last_accessed, d.file_type, d.size, d.is_pinned, d.comment, d.last_opened, d.last_synced, d.last_parsed
        FROM metadata m
        JOIN (
            SELECT id, file_type, size, is_pinned, comment, last_opened, last_synced, last_parsed
            FROM document
            {where_file_type}
        ) d ON m.source_id = d.id
        ORDER BY last_modified DESC
        LIMIT {limit} OFFSET {offset}
    "#,
        where_file_type = if !where_file_type.is_empty() {
            where_file_type
        } else {
            "".to_string()
        },
        limit = limit,
        offset = page * limit
    );
    println!("inner_query: {}", inner_query);
    let search_results = diesel::sql_query(inner_query).load::<DocumentSearchResult>(&mut conn)?;

    if search_results.len() > 0 {
        println!("search_results: {:?}", search_results[0]);
    }
    Ok(search_results)
}

// Get the counts for all file_types from the document table
pub fn get_counts_for_all_filetypes(
    mut conn: PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<Vec<DBStat>, diesel::result::Error> {
    // couldn't get COUNT -- GROUP BY to work so doing it manually for each filetype
    use crate::database::schema::document::dsl::*;
    let all_filetypes = all_allowed_filetypes(&mut conn, true);
    let mut counts: Vec<DBStat> = Vec::new();
    for doctype in all_filetypes {
        let count = document
            .filter(file_type.eq(&doctype.file_type))
            .count()
            .get_result(&mut conn)?;
        counts.push(DBStat {
            file_type: doctype.file_type,
            count: count,
        });
    }
    Ok(counts)
}

// Get counts for total files and num files parsed
pub fn get_file_parsed_count(mut conn: PooledConnection<ConnectionManager<SqliteConnection>>) -> Result<i64, diesel::result::Error> {
    use crate::database::schema::document::dsl::*;
    let parsed_files = document
        .filter(last_parsed.gt(0))
        .filter(file_type.ne("folder"))
        .count().get_result(&mut conn)?;
    Ok(parsed_files)
}

// Handle special case with NEGATIVE query only
// Get recently opened docs (which is what the user was seeing when they typed the query)
// Then filter out the results that match the negative query/queries
fn handle_special_case(
    query: String,
    page: i32,
    limit: i32,
    file_type: Option<String>,
    conn: PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<Vec<DocumentSearchResult>, diesel::result::Error> {
    let query_segments: QuerySegments = parse_stringified_query_segments(&query);
    println!("query_segments: {:?}", query_segments);
    let outer_search_results = get_recently_opened_docs(page, limit*2, file_type, conn).unwrap();
    let mut search_results: Vec<DocumentSearchResult> = Vec::new();
    // iterate over outer_search_results and remove any item where item.name or item.path contains any of query_segments.not_segments
    for result in outer_search_results {
        let mut found = false;
        for not_segment in &query_segments.not_segments {
            if result.name.contains(not_segment) || result.path.contains(not_segment) || result.file_type.contains(not_segment) {
                found = true;
                break;
            }
        }
        if !found {
            search_results.push(result);
        }
    }
    Ok(search_results)
}

// Handle special case with NEGATIVE query only
// We don't bother looking into body_fts here because it's not needed for eliminating results
fn _handle_special_case_old(
    query: String,
    page: i32,
    limit: i32,
    where_date_limit: String,
    where_file_type: String,
    mut conn: PooledConnection<ConnectionManager<SqliteConnection>>,
) -> Result<Vec<DocumentSearchResult>, diesel::result::Error> {
    let query_segments: QuerySegments = parse_stringified_query_segments(&query);
    println!("query_segments: {:?}", query_segments);

    let where_date_limit_clone = where_date_limit.clone();
    let where_file_type_clone = where_file_type.clone();

    let mut match_string = format!("(\"{}\")", query_segments.not_segments.join("\" OR \""));
    match_string = match_string.trim().to_string();
    println!("match_string: {}", match_string);

    // Run a modified query to get all documents that MATCH the query
    // Not running OFFSET here because only the top matches are needed
    // getting 5x limit results to catch OR cases that might be missed in first set
    let inner_query = format!(
        r#"
          SELECT m.source_domain, m.source_id as id, m.title as name, m.url as path, m.created_at, m.frecency_rank, m.frecency_last_accessed, d.file_type, d.size, d.is_pinned, d.comment, d.last_opened, d.last_synced, d.last_modified, d.last_parsed
          FROM metadata_fts m
          JOIN (
              SELECT id, file_type, size, is_pinned, comment, last_opened, last_synced, last_modified, last_parsed
              FROM document
              {inner_where} {where_file_type} {where_date_limit}
          ) d ON m.source_id = d.id
          {match_clause}
          LIMIT {limit}
        "#,
        inner_where = if !where_file_type.is_empty() || !where_date_limit.is_empty() {
            "WHERE".to_string()
        } else {
            "".to_string()
        },
        match_clause = if !match_string.is_empty() {
            format!(
                "WHERE metadata_fts MATCH '{}' ORDER BY bm25(metadata_fts, 1,1,1,1,50)",
                match_string
            )
        } else {
            "".to_string()
        },
        where_file_type = if !where_file_type.is_empty() {
            where_file_type
        } else {
            "".to_string()
        },
        where_date_limit = if !where_date_limit.is_empty() {
            where_date_limit
        } else {
            "".to_string()
        },
        limit = limit * 5
    );

    println!("inner_query: {}", inner_query);
    let inner_search_results: Vec<DocumentSearchResult> =
        diesel::sql_query(inner_query).load::<DocumentSearchResult>(&mut conn)?;

    // Run another query to get all documents for the given date_limit and file_type
    let outer_query = format!(
        r#"
          SELECT m.source_domain, m.source_id as id, m.title as name, m.url as path, m.created_at, m.frecency_rank, m.frecency_last_accessed, d.file_type, d.size, d.is_pinned, d.comment, d.last_opened, d.last_synced, d.last_modified, d.last_parsed
          FROM metadata_fts m
          JOIN (
              SELECT id, file_type, size, is_pinned, comment, last_opened, last_synced, last_modified, last_parsed
              FROM document
              {inner_where} {where_file_type_clone} {where_date_limit_clone}
              ORDER BY last_modified DESC
          ) d ON m.source_id = d.id
          LIMIT {limit} OFFSET {offset}
        "#,
        inner_where = if !where_date_limit_clone.is_empty() || !where_file_type_clone.is_empty() {
            "WHERE".to_string()
        } else {
            "".to_string()
        },
        where_date_limit_clone = if !where_date_limit_clone.is_empty() {
            where_date_limit_clone
        } else {
            "".to_string()
        },
        where_file_type_clone = if !where_file_type_clone.is_empty() {
            where_file_type_clone
        } else {
            "".to_string()
        },
        limit = limit,
        offset = page * limit,
    );

    println!("outer_query: {}", outer_query);
    let outer_search_results: Vec<DocumentSearchResult> =
        diesel::sql_query(outer_query).load::<DocumentSearchResult>(&mut conn)?;

    // Get the difference between the two sets of results
    let mut search_results: Vec<DocumentSearchResult> = Vec::new();
    for outer_result in outer_search_results {
        let mut found = false;
        for inner_result in &inner_search_results {
            if inner_result.id == outer_result.id {
                found = true;
                break;
            }
        }
        if !found {
            search_results.push(outer_result);
        }
    }

    Ok(search_results)
}

// Get search suggestions
pub fn get_metadata_title_matches(
    query: String,
    conn: &mut SqliteConnection,
) -> Result<Vec<String>, diesel::result::Error> {
    println!("getting suggestions for: {}!", query);
    let inner_query = format!(
        r#"
            SELECT snippet(metadata_fts, 4, '', '', '', 2) as title from metadata_fts WHERE metadata_fts MATCH '{}*' ORDER BY rank LIMIT 10;
        "#,
        query
    );
    let keyword_suggestions: Vec<MetadataFTSSearchResult> = diesel::sql_query(inner_query).load::<MetadataFTSSearchResult>(conn)?;
    let mut suggestions: Vec<String> = keyword_suggestions.iter().map(|suggestion| suggestion.title.clone()).collect();
    
    // convert keywords to lowercase
    suggestions = suggestions.iter().map(|s| s.trim().to_lowercase()).collect();
    // iterate over the suggestions and remove any item that does not contain the query
    suggestions.retain(|suggestion| suggestion.contains(&query));
    // iterate over the suggestions and remove any item that contains any character other than alphanumeric, _, - and space
    suggestions.retain(|suggestion| suggestion.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == ' '));

    // let file_suggestions = metadata::table
    //     .select(metadata::title)
    //     .filter(metadata::title.like(format!("%{}%", query)))
    //     .order(metadata::last_modified.desc())
    //     .limit(7)
    //     .load::<String>(conn)?;

    // for suggestion in &file_suggestions {
    //     suggestions.push(suggestion.clone());
    // }

    // remove duplicates
    suggestions.dedup();

    Ok(suggestions)
}

// Get parsed text for file
pub fn get_parsed_text_for_file(document_id: i32, conn: &mut SqliteConnection) -> Result<Vec<String>, diesel::result::Error> {
  // get all items from body where url is equal to file_path
  let parsed_text_rows = body::table
    .filter(body::source_id.eq(document_id))
    .select(body::text)
    .order_by(body::id.asc())
    .load::<String>(conn)
    .unwrap();

  Ok(parsed_text_rows)
}

// Get the file_id from the document table in the database
pub fn get_file_id_from_path(file_path: &String, conn: &mut SqliteConnection) -> Result<i32, diesel::result::Error> {
  use crate::database::schema::document::dsl::*;
  let file_id = document
    .select(id)
    .filter(path.eq(file_path))
    .first::<i32>(conn);

  if file_id.is_ok() {
    return Ok(file_id.unwrap());
  } else {
    return Ok(0);
  }
}