# ION

```ruby
# ion(s) | intuitive object notation (ser/deser)
College MIT
Grades {
    Student1 92
    Student2 95
    Student3 93
    Student4 98
}
Classes [
    History
    Math
    Biology
    CS
    Classics
]
```

```rust
#[derive(Serialize, Deserialize, Debug)]
pub struct Docs {
    College: String,
    Grades: Grades,
    Classes: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Grades {
    Student1: i64,
    Student2: i64,
    Student3: i64,
    Student4: i64,
}

// the current way of deserializing a struct in ion is weird since
// a deserializer hasn't been implemented yet.
// but, since ion and json are similar, ion is transpiled to json
// and can then be deserialized correctly.
// a deserializer is in progress

use std::fs::read_to_string;
use ion::ion_to_json;

fn main() {
    let docs = read_to_string("docs.ion").unwrap();
    let docs = ion_to_json(&docs).unwrap();
    println!("ion as json {}", docs);
    let docs: Docs = serde_json::from_str(&docs).unwrap();
    println!("{:#?}", docs);
}
```
