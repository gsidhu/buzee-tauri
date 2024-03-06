// use super::models::DocumentItem;
// use super::schema::document;
// use crate::database::establish_connection;
// use diesel::RunQueryDsl; // Import the RunQueryDsl trait

// pub fn add_files_to_database(files_array: Vec<DocumentItem>) {
//   println!("Adding files to database: {}", files_array[0].name);
//   let mut connection = establish_connection();
//   let _ = diesel::insert_into(document::table)
//     .values(&files_array)
//     .execute(&mut connection) // Pass the connection by mutable reference
//     .unwrap();
// }