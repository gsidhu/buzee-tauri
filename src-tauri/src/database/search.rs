use crate::custom_types::{DateLimit, DBStat};
use crate::database::models::{DocumentSearchResult, DocumentResponseModel, MetadataItem, MetadataSearchResult};
// use crate::database::response_models::DocumentResponseModel;
use crate::indexing::all_allowed_filetypes;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::SqliteConnection;

// Return documents from the document_fts Index that match the given search query (name and type)
// bm25(document_fts, 10) is the ranking function which gives 10x weight to the file name (first column)
pub fn search_fts_index(
    query: String,
    page: i32,
    limit: i32,
    file_type: Option<String>,
    date_limit: Option<DateLimit>,
    mut conn: SqliteConnection,
) -> Result<Vec<DocumentSearchResult>, diesel::result::Error> {
    println!(
        "search_fts_index: query: {}, page: {}, limit: {}, file_type: {:?}, date_limit: {:?}",
        query, page, limit, file_type, date_limit
    );
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
            if !where_file_type.is_empty() { "AND" } else { "" },
            date_limit.start, date_limit.end
        )
    } else {
        "".to_string()
    };

    println!("where_file_type: {}", where_file_type);

    // Give 5x weight to the title column (4th) in metadata_fts
    let inner_query = format!(
        r#"
          SELECT m.source_domain, m.source_id as id, m.title as name, m.url as path, m.created_at, m.frecency_rank, m.frecency_last_accessed, d.file_type, d.size, d.is_pinned, d.comment, d.last_opened, d.last_synced, d.last_modified
          FROM metadata_fts m
          JOIN (
              SELECT id, file_type, size, is_pinned, comment, last_opened, last_synced, last_modified
              FROM document
              {inner_where} {where_file_type} {where_date_limit}
          ) d ON m.source_id = d.id
          WHERE {match_clause}
          ORDER BY bm25(metadata_fts, 1,1,1,5)
          LIMIT {limit} OFFSET {offset}
        "#,
        inner_where = if !where_file_type.is_empty() || !where_date_limit.is_empty() {
          "WHERE".to_string()
        } else {
          "".to_string()
        },
        match_clause = if !query.is_empty() {
            format!("metadata_fts MATCH '{}'", query)
        } else {
            "".to_string()
        },
        where_file_type = if !where_file_type.is_empty() {
            where_file_type
        } else {
            "".to_string()
        },
        limit = limit,
        offset = page * limit,
        where_date_limit = if !where_date_limit.is_empty() {
          where_date_limit
        } else {
            "".to_string()
        }
    );

    println!("inner_query: {}", inner_query);

    let search_results: Vec<DocumentSearchResult> =
        diesel::sql_query(inner_query).load::<DocumentSearchResult>(&mut conn)?;

    if search_results.len() > 0 {
      println!("search_results: {:?}", search_results[0]);
    }

    Ok(search_results)
}

// Get recently opened documents
pub fn get_recently_opened_docs(
    page: i32,
    limit: i32,
    file_type: Option<String>,
    mut conn: SqliteConnection,
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
        SELECT m.source_domain, m.source_id as id, m.title as name, m.url as path, m.created_at, m.last_modified, m.frecency_rank, m.frecency_last_accessed, d.file_type, d.size, d.is_pinned, d.comment, d.last_opened, d.last_synced
        FROM metadata m
        JOIN (
            SELECT id, file_type, size, is_pinned, comment, last_opened, last_synced
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
    let search_results =
        diesel::sql_query(inner_query).load::<DocumentSearchResult>(&mut conn)?;

    if search_results.len() > 0 {
      println!("search_results: {:?}", search_results[0]);
    }
    Ok(search_results)
}

// Get the counts for all file_types from the document table
pub fn get_counts_for_all_filetypes(
    mut conn: SqliteConnection,
) -> Result<Vec<DBStat>, diesel::result::Error> {
  use crate::database::schema::document::dsl::*;

  let all_filetypes = all_allowed_filetypes();
  let mut counts: Vec<DBStat> = Vec::new();
  for doctype in all_filetypes {
    let count = document.filter(file_type.eq(doctype.to_string())).count().get_result(&mut conn)?;
    counts.push(DBStat {
        file_type: doctype.to_string(),
        count: count,
    });
  }
  Ok(counts)
}