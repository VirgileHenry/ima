/// Created by Virgile HENRY, 2023/09/28

use std::path::PathBuf;

use crate::{parse, IMA, ImaOptions};

#[test]
#[ignore]
fn full() {
    // read all files in ima folder
    let paths = std::fs::read_dir("D:/Dev/rust/ima/ima-core/src/test/ass").unwrap();

    let mut success = 0;
    let mut failures = Vec::new(); 

    for path in paths {
        let path = path.unwrap().path();

        println!("Running test for file {path:?}");

        match run(&path) {
            Ok(_) => success += 1,
            Err(e) => failures.push(e),
        }
    }

    if !failures.is_empty() {
        panic!("Full test failed: {}/{} test failed.", failures.len(), failures.len() + success);
    }
}

fn run(path: &PathBuf) -> Result<(), String> {
    let source_code = std::fs::read(&path).expect(&format!("Failed to read file at {:?}", path.display()));
    let source_code = String::from_utf8(source_code).expect(&format!("Unable to read source as utf8: {:?}", path.display()));
    
    let program = parse(&source_code).expect(&format!("Unable to parse file at {:?}", path.display()));
    let mut ima = IMA::new(program, ImaOptions::default());
    
    // custom input and output
    let mut input = std::io::Cursor::new(b"12");
    let mut output = Vec::new();
    
    let res = ima.run(&mut input, &mut output);
    
    res.map_err(|e| format!("{e:?}"))
}