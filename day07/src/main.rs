use std::fmt::Display;
use std::{fs::File, io::Read};

#[derive(Debug)]
struct Game<'hand> {
    hand: &'hand str,
    joker_enabled: bool, // needed for part 2
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
                // if joker is enabled, J card is the weakest
                // this is only important for comparing hands with the same handtype
                let order = match self.joker_enabled {
                    true => "J23456789TQKA",
                    false => "23456789TJQKA",
                };
                for (s, o) in self.hand.chars().zip(other.hand.chars()) {
                    return match order.find(s).unwrap().cmp(&order.find(o).unwrap()) {
                        Less => Less,
                        Equal => {
                            continue;
                        }
                        Greater => Greater,
                    };
                }
                unreachable!("No two games can be equal.");
            }
            Greater => Greater,
        }
    }
}

fn strength(hand: &str, joker_enabled: bool) -> HandType {
    let mut unique = hand.as_bytes().to_owned();
    use HandType::*;
    //Joker is not enabled, or it is enabled but our hand has no J cards,
    //which means we will continue as normal
    if !joker_enabled {
        unique.sort();
        unique.dedup();
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
            0 => {
                //special case, all we have is Jokers, and we checkes the length without jokers in a joker_enaled game
                //so it's Five Jokers
                Five
            }
            _ => unreachable!("Can't have more than 5 cards"),
        }
    } else {
        let joker_count = unique.iter().filter(|&&t| t == b'J').count() as u8;
        // if we have no jokers, we don't care about card mimicking,
        // just use the strength function as if we don't have joker enabled.
        if joker_count == 0 {
            return strength(hand, !joker_enabled);
        };
        //we have at least one joker in our hand and joker_enabled is true
        assert!(joker_enabled && joker_count >= 1);
        // find the hand's strength without joker count, and then add joker_count
        // to the max card in order to increase the hand's strength
        let joker_filtered: Vec<u8> = unique
            .iter()
            .filter_map(|&t| if t != b'J' { Some(t) } else { None })
            .collect();
        let joker_filtered = std::str::from_utf8(&joker_filtered).unwrap();
        strength_with_jokers(joker_filtered, joker_count)
    }
}
fn strength_with_jokers(joker_filtered: &str, joker_count: u8) -> HandType {
    let mut unique = joker_filtered.as_bytes().to_owned();
    unique.sort();
    unique.dedup();
    let hand = joker_filtered.as_bytes();
    let max_occurrences = hand
        .iter()
        .map(|ch| hand.iter().filter(|&c| c == ch).count())
        .max()
        .unwrap_or(0);
    match unique.len() {
        0 | 1 => HandType::Five,
        2 => {
            // we have two unique cards without joker
            // find max count and add jokers to it
            match max_occurrences {
                1 => {
                    // we have two unique cards other than Jokers
                    // and max occurence is one, so we have to have 3 jokers
                    assert_eq!(
                        joker_count, 3,
                        "Something's wrong I can feel it. Read comments."
                    );
                    HandType::Four
                }
                2 => {
                    // we have 2 unique cards, and one of them have an occurence of 2.
                    //however they can both have 2 occurences,
                    //Example: 2233J  has 2 unique, but also 2 twice occurence
                    //or 22JJ3 has 2 unique but 1 max occurence of 2.
                    match joker_count {
                        1 => {
                            //two times twice occurence,
                            HandType::FullHouse
                        }
                        2 => HandType::Four,
                        _ => unreachable!(),
                    }
                }
                3 => HandType::Four,
                _ => unreachable!(),
            }
        }
        3 => {
            match max_occurrences {
                1 | 2=> {
                    //3 different cards all have 1 occurence, 2 jokers
                    //XYZJJ
                    //or 3 different cards, one has 2 occurence + 1 joker
                    //XXYZJ
                    HandType::Three
                }
                _ => unreachable!("Can't have 3 different cards without jokers and three occurence of one of them at the same time.")
            }
        }
        4 => {
            //we have 4 different cards without jokers,
            // one joker must be present
            assert_eq!(joker_count, 1);
            HandType::OnePair
        }
        _ => HandType::HighCard,
    }
}

fn main() {
    // Read the contents of the file into a string
    let mut text = String::new();
    let res = File::open("input.txt")
        .expect("Error: Input file not found")
        .read_to_string(&mut text)
        .unwrap();
    assert!(res > 0, "Error: Input file is empty");

    println!("{}", part1(&text));
    println!("{}", part2(&text));
}

fn part1(text: &str) -> usize {
    let mut games: Vec<Game> = text
        .split('\n')
        .map(|line| {
            let (hand, bid_str) = line.split_once(' ').unwrap();
            Game {
                hand,
                joker_enabled: false,
                bid: bid_str.parse().unwrap(),
                hand_type: strength(hand, false),
            }
        })
        .collect();
    games.sort();
    games
        .iter()
        .enumerate()
        .fold(0, |sum, (i, game)| sum + ((i + 1) * game.bid))
}

fn part2(text: &str) -> usize {
    let mut games: Vec<Game> = text
        .split('\n')
        .map(|line| {
            let (hand, bid_str) = line.split_once(' ').unwrap();
            Game {
                hand,
                joker_enabled: true,
                bid: bid_str.parse().unwrap(),
                hand_type: strength(hand, true),
            }
        })
        .collect();
    games.sort();
    games
        .iter()
        .enumerate()
        .fold(0, |sum, (i, game)| sum + ((i + 1) * game.bid))
}
