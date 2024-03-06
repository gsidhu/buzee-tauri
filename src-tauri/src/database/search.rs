use diesel::SqliteConnection;
use diesel::RunQueryDsl;

use crate::database::models::SearchResult;
// Return documents from the document_fts Index that match the given search query (name and type)
// bm25(document_fts, 10) is the ranking function which gives 10x weight to the file name (first column)

pub fn search_fts_index(
    query: &str,
    page: i32,
    limit: i32,
    file_type: Option<&str>,
    mut conn: SqliteConnection,
) -> Result<Vec<SearchResult>, diesel::result::Error> {
    let mut where_condition = ""; // Initialize where_condition

    // Add type(s)
    if let Some(file_type) = file_type {
        if !file_type.contains(",") {
            where_condition = &format!(r#" AND filetype IN ('{}')"#, file_type);
        } else {
            where_condition = &format!(r#" AND filetype IN ({})"#, file_type.replace(",", "','"));
        }
    }

    let inner_query = format!(
        r#"
        SELECT d.name, d.path, d.size, d.file_type, d.last_modified, d.last_opened, d.created_at
        FROM document d
        JOIN (
            SELECT DISTINCT path
            FROM document_fts
            WHERE {match_clause}
            ORDER BY bm25(document_fts, 10)
            LIMIT {limit} OFFSET {offset}
        ) t ON d.path = t.path
        "#,
        match_clause = if !query.is_empty() {
            format!("document_fts MATCH '{}'", query)
        } else {
            "".to_string()
        },
        limit = limit,
        offset = page * limit
    );

    println!("inner_query: {}", inner_query);

    let search_results: Vec<SearchResult> = diesel::sql_query(inner_query).load::<SearchResult>(&mut conn)?;

    println!("search_results: {:?}", search_results);
    Ok(search_results)
}
