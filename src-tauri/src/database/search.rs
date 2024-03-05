// use crate::utils::*;
// use crate::database::*;
// use crate::database::models::SearchResult;

// Have to use raw SQL queries for searching the FTS tables because –
// "TypeORM's addOrderBy/orderBy methods expect the first argument to be
// a column name or an alias, not a function call. Unfortunately, TypeORM does
// not support function calls in the addOrderBy method."

// Return documents from the document_fts Index that match the given search query (name and type)
// bm25(document_fts, 10) is the ranking function which gives 10x weight to the file name (first column)
// pub fn search_fts_index(
//   query: String,
//   page: int,
//   limit: int,
//   filetype: Optional(String),
//   date_limit: Optional(DateLimit)
// ) -> Vec<SearchResult> {
//     let con = establish_connection();
//     let mut where_condition: &str;
//     // Add type(s)
//     if (filetype && filetype.indexOf(",") < 0) {
//       where_condition = format!("filetype IN (" + "{}" + ")", FileType::new(type).name);
//     }
//     if (filetype && filetype.indexOf(",") > -1) {
//       where_condition = format!("filetype IN ({})", FileType::new(type).name);
//     }
//     // Add date limit 
//     if (date_limit) {
//       if (where_condition.length > 0) {
//         where_condition += r#" AND "#;
//       }
//       where_condition += r#"last_modified >= date('${dateLimit.start}') 
//       AND last_modified <= date('${dateLimit.end}') "#;
//     }
//     let inner_query : &str = r#"
//         SELECT d.id, d.name, d.path, d.size, d.type, d.last_modified
//         FROM document d
//         JOIN (
//             SELECT DISTINCT document_id
//             FROM document_fts
//             WHERE ${query !== '' ? `document_fts MATCH '${query}'` : ''} ${whereCondition.length > 0 ? ((query !== '') ? `AND ${whereCondition}` : `${whereCondition}`) : ''}
//             ORDER BY bm25(document_fts, 10)
//             LIMIT ${limit} OFFSET ${page * limit}
//         ) t ON d.id = t.document_id
//     "#;

//     let search_results = diesel::sql_query(inner_query.to_string()).execute(&mut con)?;
//     Ok(search_results)
// }