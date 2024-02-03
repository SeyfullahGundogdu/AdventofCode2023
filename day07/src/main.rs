use std::fmt::Display;
use std::{fs::File, io::Read};

#[derive(Debug)]
struct Game<'hand> {
    hand: &'hand str,
    bid: usize,
    hand_type: HandType,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    Five = 7,
    Four = 6,
    FullHouse = 5,
    Three = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl<'h> Display for Game<'h> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:#?}", self.hand, self.hand_type)
    }
}
impl<'h> PartialOrd for Game<'h> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<'h> PartialEq for Game<'h> {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}
impl<'h> Eq for Game<'h> {}

impl<'h> Ord for Game<'h> {
    /// compare hands for two games
    /// if their hand type is different, just return the comparison of hand types
    /// which we handled by giving each variant of the HandType enum a value
    /// if the HandType is same, compare the hand of games
    /// Ordering: A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, 2
    /// where A is the highest and 2 is the lowest.
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;

        match self.hand_type.cmp(&other.hand_type) {
            Less => Less,
            Equal => {
                let order = "23456789TJQKA";
                for (s, o) in self.hand.chars().zip(other.hand.chars()) {
                    return match order.find(s).unwrap().cmp(&order.find(o).unwrap()) {
                        Less => Less,
                        Equal => {
                            continue;
                        }
                        Greater => Greater,
                    };
                }
                unreachable!("No game can be equal.");
            }
            Greater => Greater,
        }
    }
}

fn strength(hand: &str) -> HandType {
    let mut unique = hand.as_bytes().to_owned();
    unique.sort();
    unique.dedup();
    use HandType::*;
    match unique.len() {
        5 => HighCard, // all cards are different from each other
        4 => OnePair,  // only one pair
        3 => {
            // special case, it could be two pair or three
            let hand = hand.as_bytes();
            // for each card, find the total number of cards that match it,
            // meaning number of occurences,
            // and then get the maximum of that value
            // if it's two, meaning there are three cards, but max only occurs twice
            // our hand is XXYYZ
            // if it's three our hand is XXXYZ
            let max_occurrences = hand
                .iter()
                .map(|ch| hand.iter().filter(|&c| c == ch).count())
                .max()
                .unwrap();

            if 3 == max_occurrences {
                Three
            } else {
                TwoPair
            }
        }
        2 => {
            let hand = hand.as_bytes();
            // same thing as above but now we check if max occurence is 4
            // if so our hand is XXXXY
            // else it's XXXYY
            let max_occurrences = hand
                .iter()
                .map(|ch| hand.iter().filter(|&c| c == ch).count())
                .max()
                .unwrap();

            if 4 == max_occurrences {
                Four
            } else {
                FullHouse
            }
        }
        1 => Five,
        _ => panic!("Non-Specified Hand for the Game."),
    }
}

fn main() {
    // Read the contents of the file into a string
    let mut text = String::new();
    let res = File::open("sample.txt")
        .expect("Error: Input file not found")
        .read_to_string(&mut text)
        .unwrap();
    assert!(res > 0, "Error: Input file is empty");
    let mut games: Vec<Game> = text
        .split('\n')
        .map(|line| {
            let (hand, bid_str) = line.split_once(' ').unwrap();
            Game {
                hand,
                bid: bid_str.parse().unwrap(),
                hand_type: strength(hand),
            }
        })
        .collect();
    games.sort();
    let sum = games.iter().enumerate().fold(0, |sum, (i, game)| {
        sum + ((i + 1) * game.bid)
    });
    println!("{}", sum);
}
