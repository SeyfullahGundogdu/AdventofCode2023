use std::{fs::File, io::Read};

const RED: i32 = 12;
const GREEN: i32 = 13;
const BLUE: i32 = 14;

fn main() {
    //get the file into a string
    let mut text = String::new();
    let res = File::open("input.txt")
        .expect("input file not found")
        .read_to_string(&mut text)
        .unwrap();
    assert!(res > 0, "input file empty");

    //split the file at newline and remove the last empty line at the end of the file
    let lines: Vec<&str> = text.split('\n').filter(|line| !line.is_empty()).collect();

    let p1_sum = part1(&lines);
    let p2_sum = part2(&lines);
    println!("Part1: {p1_sum}");
    println!("Part2: {p2_sum}");
}

fn is_game_valid(game: &str) -> bool {
    //get each grab by splitting the game at semicolons
    let games: Vec<&str> = game.split("; ").collect();
    for grabs in games {
        // again split at , to get each color seperately
        let colors: Vec<&str> = grabs.split(", ").collect();

        for elem in colors {
            // again, split at whitespace to get the number and color seperatly
            let (count, color) = elem.split_once(' ').unwrap();

            let count = count.parse::<i32>().unwrap();
            match color {
                "red" => {
                    //if red count is impossible
                    if count > RED {
                        return false;
                    }
                }
                "green" => {
                    //if green count is impossible
                    if count > GREEN {
                        return false;
                    }
                }
                "blue" => {
                    //if blue count is impossible
                    if count > BLUE {
                        return false;
                    }
                }
                _ => {}
            }
        }
    }
    // every game variant was valid, return true
    true
}

fn part1(lines: &Vec<&str>) -> i32 {
    let mut sum = 0;
    //parse each line
    for line in lines {
        //split the line at colon and get the game id
        let (game_count, game) = line.split_once(": ").unwrap();

        //line is split into 2 pieces,
        //get the game id from the &str at first index of [linesplit]
        //[Game ..], game number starts at the index 5, parse through the end.
        //parse the game itself
        if is_game_valid(game) {
            let game_id = game_count[5..].parse::<i32>().unwrap();
            sum += game_id;
        }
    }
    sum
}

fn part2(lines: &Vec<&str>) -> i32 {
    let mut sum = 0;
    // parse each line
    for line in lines {
        sum += game_power(line);
    }
    sum
}

fn game_power(line: &str) -> i32 {
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;
    // split the line at colon and get game
    let (_, game) = line.split_once(": ").unwrap();
    let games: Vec<&str> = game.split("; ").collect();
    for grabs in games {
        // again split at , to get each color seperately
        let colors: Vec<&str> = grabs.split(", ").collect();

        for elem in colors {
            // again, split at whitespace to get the number and color seperatly
            let (count, color) = elem.split_once(' ').unwrap();

            let count = count.parse::<i32>().unwrap();
            match color {
                "red" => {
                    min_red = min_red.max(count);
                }
                "green" => {
                    min_green = min_green.max(count);
                }
                "blue" => {
                    min_blue = min_blue.max(count);
                }
                _ => {}
            }
        }
    }
    min_red * min_green * min_blue
}
