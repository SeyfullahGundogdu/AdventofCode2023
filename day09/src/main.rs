use std::{fs::File, io::Read};

fn main() {
    let mut text = String::new();
    let res = File::open("input.txt")
        .expect("Error: Input file not found")
        .read_to_string(&mut text)
        .unwrap();
    assert!(res > 0, "Error: Input file is empty");
    println!("Part 1: {}", part1(&text));
    println!("Part 2: {}", part2(&text));
}

// get the sum of a vector, if it's not 0, go one level deeper using the differences between the elements,
// if we look forward, add the sum to the last element of each vector,
// if not, subtract the
fn parse_line(line: &[i64], forward: bool) -> i64 {
    // some of the elements are not 0, go deeper
    if !line.iter().all(|&n| n == 0) {
        // iterate over the vector, zip it with its tail, and calculate the differences
        let differences: Vec<_> = line
            .iter()
            .zip(line.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect();
        if forward {
            return parse_line(&differences, forward) + line.last().unwrap();
        } else {
            return line.first().unwrap() - parse_line(&differences, forward);
        }
    }
    0 //base case, all elemnts are 0, sum of them is 0
}

fn part1(text: &str) -> i64 {
    text.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|line| parse_line(&line, true))
        .sum()
}

fn part2(text: &str) -> i64 {
    text.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|line| parse_line(&line, false))
        .sum()
}
