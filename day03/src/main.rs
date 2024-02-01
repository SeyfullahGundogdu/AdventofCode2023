use std::{fs::File, io::Read};
#[derive(Debug, Clone, Copy)]
struct Number {
    value: u32,
    coordinates: (usize, usize),
    len: usize,
}

#[derive(Debug, Clone, Copy)]
struct Symbol {
    sym: char,
    coordinates: (usize, usize),
}
impl Number {
    fn near_points(&self) -> Vec<(usize, usize)> {
        let mut near_points = vec![];
        let top_left = (
            if self.coordinates.0 > 0 {
                self.coordinates.0 - 1
            } else {
                0
            },
            if self.coordinates.1 > 0 {
                self.coordinates.1 - 1
            } else {
                0
            },
        );
        let bottom_right = (self.coordinates.0 + 1, self.coordinates.1 + self.len);
        for y in top_left.0..=bottom_right.0 {
            for x in top_left.1..=bottom_right.1 {
                near_points.push((y, x));
            }
        }
        near_points
    }
    fn is_a_part_number(&self, symbol_coord: &Symbol) -> bool {
        self.near_points().contains(&symbol_coord.coordinates)
    }
}
fn main() {
    //get the file into a string
    let mut text = String::new();
    let res = File::open("input2.txt")
        .expect("input file not found")
        .read_to_string(&mut text)
        .unwrap();
    assert!(res > 0, "input file empty");

    //split the file at newline and remove the last empty line at the end of the file
    let lines: Vec<&str> = text.split('\n').filter(|line| !line.is_empty()).collect();
    let mut numbers_in_the_file: Vec<Vec<Number>> = vec![];
    for (y, line) in lines.iter().enumerate() {
        let numbers = extract_numbers_in_line(line, y);

        numbers_in_the_file.push(numbers);
    }

    let mut symbols_in_the_file = vec![];
    for (y, line) in lines.iter().enumerate() {
        let symbols = extract_symbols_in_line(line, y);
        symbols_in_the_file.push(symbols);
    }
    let numbers: Vec<Number> = numbers_in_the_file
        .iter()
        .flat_map(|n| n.iter().cloned())
        .collect();
    let symbols: Vec<Symbol> = symbols_in_the_file
        .iter()
        .flat_map(|s| s.iter().cloned())
        .collect();
    let star_symbols: Vec<Symbol> = symbols
        .iter()
        .filter_map(|&symbol| {
            if symbol.sym == '*' {
                Some(symbol)
            } else {
                None
            }
        })
        .collect();

    //part1
    let part_num_sum = find_part_numbers(&numbers, &symbols);
    println!("{:?}", part_num_sum);
    //part2
    let gear_ratio_sum = find_gear_ratios(&numbers, &star_symbols);
    println!("{}", gear_ratio_sum);
}

fn extract_numbers_in_line(line: &str, y_axis: usize) -> Vec<Number> {
    let mut nums = vec![];
    let mut num = 0;
    let line = line.chars().collect::<Vec<char>>();
    for i in 0..line.len() {
        if line[i].is_numeric() {
            num *= 10;
            num += line[i] as u32 - 48;
        }
        if !line[i].is_numeric() && num > 0 {
            let num_len = num.to_string().len();
            nums.push(Number {
                value: num,
                coordinates: (y_axis, i - num_len),
                len: num_len,
            });
            num = 0;
        }
        if num > 0 && i == line.len() - 1 {
            let num_len = num.to_string().len();
            nums.push(Number {
                value: num,
                coordinates: (y_axis, i - num_len),
                len: num_len,
            });
            num = 0;
        }
    }
    nums
}

fn extract_symbols_in_line(line: &str, y_axis: usize) -> Vec<Symbol> {
    line.chars()
        .enumerate()
        .filter_map(|(i, ch)| {
            if ch != '.' && ch.is_ascii_punctuation() {
                Some(Symbol {
                    sym: ch,
                    coordinates: (y_axis, i),
                })
            } else {
                None
            }
        })
        .collect()
}

fn find_part_numbers(numbers: &[Number], symbols: &[Symbol]) -> u32 {
    let mut sum = 0;

    for num in numbers {
        if symbols.iter().any(|sym| num.is_a_part_number(sym)) {
            sum += num.value;
        }
    }
    sum
}

fn find_gear_ratios(numbers: &[Number], symbols: &[Symbol]) -> u32 {
    let mut gear_sum = 0;
    for symbol in symbols {
        let nums_near = numbers
            .iter()
            .filter(|num| num.near_points().contains(&symbol.coordinates))
            .collect::<Vec<&Number>>();
        if nums_near.len() == 2 {
            gear_sum += nums_near[0].value * nums_near[1].value;
        }
    }
    gear_sum
}
