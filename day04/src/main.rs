use std::{fs::File, io::Read};

fn main() {
    // get the file into a string
    let mut text = String::new();
    let res = File::open("input.txt")
        .expect("input file not found")
        .read_to_string(&mut text)
        .unwrap();
    assert!(res > 0, "input file empty");

    // split the file at newline and remove the last empty line at the end of the file
    let lines: Vec<Vec<&str>> = text
        .split('\n')
        .filter_map(|line| {
            if !line.is_empty() {
                let line = line.split(": ").collect::<Vec<&str>>()[1];
                Some(line.split("| ").collect())
            } else {
                None
            }
        })
        .collect();

    // vector used for  part 2
    let mut cards = vec![];

    // Part 1
    let mut total_points = 0;
    for (i, line) in lines.iter().enumerate() {
        let winner_cards: Vec<&str> = line[0].split_whitespace().collect();
        let my_cards: Vec<&str> = line[1].split_whitespace().collect();

        let number_of_matches = my_cards
            .iter()
            .filter(|&c| winner_cards.contains(c))
            .collect::<Vec<&&str>>()
            .len();

        // for part 2
        cards.push((i, number_of_matches));
        if number_of_matches > 0 {
            total_points += 2u32.pow(number_of_matches as u32 - 1);
        }
    }
    println!("Part 1: total points = {}", total_points);

    // get a vector with the same size as our card table,
    // which is the number of lines in our input text
    let mut total_cards: Vec<usize> = vec![1; cards.len()];

    // now for each line, find how many times our next line of cards will be repeated
    for card in cards.iter().enumerate() {
        // for the next lines from 1 to our matching count,
        // which is stored in the second variable in our tuple
        // hence card.1.1
        for i in 1..=card.1 .1 {
            // if we have n total copies of our cards in the current line,
            // it means we will have n more copies of next 1..card.1.1 cards
            // it could panic if card.0 + i is bigger than our line count,
            // meaning that the matching numbers count + line's position exceeds the number of lines in out input
            // so we do a bound check
            if card.0 + i < total_cards.len() {
                total_cards[card.0 + i] += total_cards[card.0];
            }
        }
    }
    println!(
        "Total number of Scratchcards: {:?}",
        total_cards.iter().sum::<usize>()
    );
}
