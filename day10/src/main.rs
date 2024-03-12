use std::{fs::File, io::Read};

fn main() {
    let mut text = String::new();
    let res = File::open("sample.txt")
        .expect("Error: Input file not found")
        .read_to_string(&mut text)
        .unwrap();
    assert!(res > 0, "Error: Input file is empty");
    
}

