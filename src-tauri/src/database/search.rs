use diesel::SqliteConnection;
use diesel::RunQueryDsl;
use crate::database::models::SearchResult;
// Return documents from the document_fts Index that match the given search query (name and type)
// bm25(document_fts, 10) is the ranking function which gives 10x weight to the file name (first column)

pub fn search_fts_index(
    query: String,
    page: i32,
    limit: i32,
    filetype: Option<String>,
    mut conn: SqliteConnection,
) -> Result<Vec<SearchResult>, diesel::result::Error> {
    println!("search_fts_index: query: {}, page: {}, limit: {}, filetype: {:?}", query, page, limit, filetype);
    // Add file type(s)
    let where_condition = if let Some(filetype) = filetype {
        if !filetype.contains(",") {
            format!(r#" AND file_type IN ('{}')"#, filetype)
        } else {
            format!(r#" AND file_type IN ('{}')"#, filetype.replace(",", "','").replace("' ", "'"))
        }
    } else {
        "".to_string()
    };

    println!("where_condition: {}", where_condition);

    let inner_query = format!(
        r#"
        SELECT d.name, d.path, d.size, d.file_type, d.last_modified, d.last_opened, d.created_at
        FROM document d
        JOIN (
            SELECT DISTINCT path
            FROM document_fts
            WHERE {match_clause}{where_condition}
            ORDER BY bm25(document_fts, 10)
            LIMIT {limit} OFFSET {offset}
        ) t ON d.path = t.path
        "#,
        match_clause = if !query.is_empty() {
            format!("document_fts MATCH '{}'", query)
        } else {
            "".to_string()
        },
        where_condition=if !where_condition.is_empty() {
            where_condition
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
