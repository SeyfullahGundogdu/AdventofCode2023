use std::{collections::HashMap, fs::File, io::Read};

fn main() {
    let word_to_number = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    let mut text = String::new();
    let res = File::open("input.txt")
        .expect("input file not found")
        .read_to_string(&mut text)
        .unwrap();
    assert!(res > 0, "input file empty");
    let lines: Vec<&str> = text.split_ascii_whitespace().collect();
    let mut sum = 0;
    for line in lines.iter() {
        let mut first: (i32, usize) = (0, usize::MAX);
        let mut last: (i32, i32) = (0, -1);

        for i in word_to_number.keys() {
            if let Some(j) = line.find(i) {
                if j < first.1 {
                    first = (*word_to_number.get(i).unwrap(), j)
                }
            }
        }
        for i in word_to_number.keys() {
            if let Some(j) = line.rfind(i) {
                if j as i32 > last.1 {
                    last = (*word_to_number.get(i).unwrap(), j as i32)
                }
            }
        }
        sum += first.0 * 10 + last.0
    }
    println!("{}", sum);
}
