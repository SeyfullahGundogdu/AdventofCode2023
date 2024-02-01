use std::{fs::File, io::Read};
use std::ops::RangeInclusive;

fn main() {
    // Read the contents of the file into a string
    let mut text = String::new();
    let res = File::open("input.txt")
        .expect("Error: Input file not found")
        .read_to_string(&mut text)
        .unwrap();
    assert!(res > 0, "Error: Input file is empty");

    let chunks: Vec<&str> = text.split("\n\n").collect();
    let seeds = chunks[0].split('\n').collect::<Vec<&str>>()[0];

    // get seeds
    let initial_seeds: Vec<u64> = seeds.split(": ").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    // Part 1
    // Iterate through the remaining chunks to apply mappings and update the seed values
    let p1_ans = parse_chunks_part1(&chunks, &initial_seeds);

    // Part 2
    // now our seeds are pair of starts and ranges,
    // rewrite initial_seeds array accordingly
    let seed_ranges: Vec<u64> = seeds.split(": ").collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let mut initial_seeds = vec![];
    for i in (0..seed_ranges.len()).step_by(2) {
        initial_seeds.push((seed_ranges[i], seed_ranges[i + 1]));
    }
    let p2_ans = parse_chunks_part2(&chunks, &initial_seeds);
}

fn parse_chunks_part1(chunks: &[&str], initial_seeds: &Vec<u64>) -> u64 {
    let mut initial_seeds = initial_seeds.to_owned();
    for chunk in &chunks[1..] {
        // Create a copy of the original seeds for each chunk
        let mut mapped_seeds = initial_seeds.clone();

        // Iterate through non-empty lines in the chunk starting from the second line
        for line in chunk.lines().filter(|l| !l.is_empty()).skip(1) {
            // Parse triplets of values from the line in the format [destination, source, range]
            let map: Vec<u64> = line
                .split_whitespace()
                .map(|m| m.parse::<u64>().unwrap())
                .collect();
            let (dest, src, range) = (map[0], map[1], map[2]);

            // Update mapped seeds based on the specified range and mapping
            for (i, &seed) in initial_seeds.iter().enumerate() {
                if seed >= src && seed < src + range {
                    // Calculate the new mapped value based on the distance from the source
                    mapped_seeds[i] = dest + (seed - src);
                }
            }
        }
        // Update the original seeds with the values from the mapped seeds
        initial_seeds = mapped_seeds;
    }

    // Return the final result, which is the minimum value in the updated seed list
    *initial_seeds.iter().min().unwrap()
}

fn parse_chunks_part2(chunks: &[&str], initial_seeds: &Vec<(u64, u64)>) -> u64 {
    // let t = RangeInclusive::new(start, end)
    todo!()
}
