use crate::custom_types::DateLimit;
use diesel::dsl::date;
use diesel::SqliteConnection;
use diesel::RunQueryDsl;
use crate::database::models::SearchResult;
// Return documents from the document_fts Index that match the given search query (name and type)
// bm25(document_fts, 10) is the ranking function which gives 10x weight to the file name (first column)

pub fn search_fts_index(
    query: String,
    page: i32,
    limit: i32,
    file_type: Option<String>,
    date_limit: Option<DateLimit>,
    mut conn: SqliteConnection,
) -> Result<Vec<SearchResult>, diesel::result::Error> {
    println!("search_fts_index: query: {}, page: {}, limit: {}, file_type: {:?}, date_limit: {:?}", query, page, limit, file_type, date_limit);
    // Add file type(s)
    let where_file_type = if let Some(file_type) = file_type {
        if !file_type.contains(",") {
            format!(r#" AND file_type IN ('{}')"#, file_type)
        } else {
            format!(r#" AND file_type IN ('{}')"#, file_type.replace(",", "','").replace("' ", "'"))
        }
    } else {
        "".to_string()
    };
    // Add date limit(s)
    let where_date_limit: String = if let Some(date_limit) = date_limit {
        format!(
            r#" WHERE last_modified >= '{}' AND last_modified <= '{}'"#,
            date_limit.start, date_limit.end
        )
    } else {
        "".to_string()
    };

    println!("where_file_type: {}", where_file_type);

    let inner_query = format!(
        r#"
        SELECT d.name, d.path, d.size, d.file_type, d.last_modified, d.last_opened, d.created_at
        FROM document d
        JOIN (
            SELECT DISTINCT path
            FROM document_fts
            WHERE {match_clause}{where_file_type}
            ORDER BY bm25(document_fts, 10)
            LIMIT {limit} OFFSET {offset}
        ) t ON d.path = t.path
        {where_date_limit}
        "#,
        match_clause = if !query.is_empty() {
            format!("document_fts MATCH '{}'", query)
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

    let search_results: Vec<SearchResult> = diesel::sql_query(inner_query).load::<SearchResult>(&mut conn)?;

    println!("search_results: {:?}", search_results);
    Ok(search_results)
}
