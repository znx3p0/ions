

#[test]
fn test() {

    let p = Test {
        s: 2,
        p: vec![
            None, None,None, Some(Box::new(Test{s: 9, p: vec![None]})), None
        ]
    };
    let p = ions::to_string(&p).unwrap();
    std::fs::write("tests/test.ion", p).unwrap();

    let p = std::fs::read_to_string("tests/test.ion").unwrap();
    let p = ions::ion_to_json(&p).unwrap();
    println!("{}", p)
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
struct Test {
    s: i32,
    p: Vec<Option<Box<Test>>>
}


