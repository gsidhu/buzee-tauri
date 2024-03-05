// // use std::fmt::{self, Debug};

// #[derive(Debug)]
// struct FileType {
//   name: String,
// }

// impl FileType {
//   fn new(name: &str) -> Self {
//     FileType {
//       name: name.to_string(),
//     }
//   }
// }
// // struct DateTime {
// //   start_date: String,
// //   end_date: String,
// //   text: String
// // }

// // impl DateTime {
// //   fn new(name: &str) -> Self {
// //     DateTime {
// //       start_date: start_date.to_string(),
// //       end_date: end_date.to_string(),
// //       text: text.to_string(),
// //     }
// //   }
// // }

// pub fn get_file_ext(file_name: &str) -> &str {
//   if !file_name.contains(".") {
//     return "";
//   }
//   file_name.split(".").last().unwrap_or("")
// }

// pub fn get_filename_from_path(x: String) -> String {
//   let path = std::path::Path::new(&x);
//   let file_stem = path.file_stem().unwrap();
//   file_stem.to_str().unwrap().to_string()
// }