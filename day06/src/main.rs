use std::{fs::File, io::Read};

pub fn main() {
    // Read the contents of the file into a string
    let mut text = String::new();
    let res = File::open("input.txt")
        .expect("Error: Input file not found")
        .read_to_string(&mut text)
        .unwrap();
    assert!(res > 0, "Error: Input file is empty");
    //part 1
    let (times_line, distances_line) = text.trim().split_once('\n').unwrap();
    let times: Vec<u64> = parse_line_p1(times_line);
    let distances: Vec<u64> = parse_line_p1(distances_line);
    let product = calculate_chances_product(&times, &distances);
    println!("{}", product);

    //part2
    let time_merged = parse_line_p2(times_line);
    let distance_merged = parse_line_p2(distances_line);

    // we use the same function to calculate correct games' product,
    // but this time we only have one game, so we just create an array inplace to put our time and distance values
    // instead of writing another fn just to have u64 arguments instead of [u64] arguments
    let product_p2 = calculate_chances_product(&[time_merged], &[distance_merged]);
    println!("{}", product_p2);
}

///
/// Parse the line, get the part after colon, split by spaces and parse the list into a [`Vec<u64>`]
///
pub fn parse_line_p1(line: &str) -> Vec<u64> {
    line.split(':')
        .nth(1)
        .unwrap_or("")
        .split(' ')
        .filter_map(|t| t.parse().ok())
        .collect()
}

///
/// Parse the line, get the part after colon, remove all spaces and parse the combined number into [u64]
///
pub fn parse_line_p2(line: &str) -> u64 {
    line.split(':')
        .nth(1) // part after colon
        .unwrap_or("1")
        .replace(' ', "")
        .parse::<u64>()
        .unwrap()
}

///
/// Calculate the total points by getting the product of points gained at each game
/// points gained in each game is the number of plays in which we beat the corresponding distance record
///
/// # Arguments:
/// 
/// * `times` - Vector of u64s which corresponds to the total time u can have in each game
///
/// * `distances` - Vector of u64s which corresponds to the record distance we should beat for each game
///  
/// times and distances vectors must be same length
pub fn calculate_chances_product(times: &[u64], distances: &[u64]) -> u64 {
    assert_eq!(
        times.len(),
        distances.len(),
        "Times and distances vectors should have the same length"
    );
    let mut chances: Vec<u64> = vec![];
    for (&time, &distance) in times.iter().zip(distances.iter()) {
        let mut begin = 0;
        while begin * (time - begin) <= distance {
            begin += 1;
        }
        // the first begin - 1 moves and the same length from the end would fail,
        // there are total of time + 1 moves( adding 0 milliseconds to the equation)
        // for 7 seconds 2mm/ms * (7-2)= 5ms = 10mm > 9mm
        // so, 2:5-3:4-4:3-5:2 are valid speed:time pairs
        // meaning the first 2 and last 2 cases are eliminated
        // and there are 7 + 1 cases
        // hence 7(time) + 1 - 2 * (beginning point that we start to win)
        chances.push(time + 1 - 2 * begin);
    }
    chances.iter().product()
}
