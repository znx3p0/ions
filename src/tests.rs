// #![allow(unused, non_snake_case)]
// use serde::{Serialize, Deserialize};

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Docs {
//     College: String,
//     Grades: Grades,
//     Classes: Vec<String>,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Grades {
//     Student1: i64,
//     Student2: i64,
//     Student3: i64,
//     Student4: i64,
// }

// the current way of deserializing a struct in ion is weird since
// a deserializer hasn't been implemented yet.
// but, since ion and json are similar, ion is transpiled to json
// and can then be deserialized correctly.
// a deserializer is in progress

// use std::fs::read_to_string;
// use ions::json_from_str;

// #[test]
// fn test() {
//     let docs = read_to_string("example.ion").unwrap();
//     let docs = json_from_str(&docs).unwrap();
//     println!("{}", docs);
//     let docs: Docs = serde_json::from_str(&docs).unwrap();
//     println!("{:#?}", docs);
// }