use std::{fs::File, io::Read, time::Instant};

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
    let mut maps = vec![vec![]; 7];
    for (i, chunk) in chunks.iter().enumerate() {
        chunk.trim().split('\n').skip(1).for_each(|triplet| {
            let triplet: Vec<u64> = triplet.split(' ').map(|num| num.parse().unwrap()).collect();
            maps[i - 1].push((triplet[0], triplet[1], triplet[2]));
        });
    }
    // Part 1
    // Iterate through the remaining chunks to apply mappings and update the seed values
    let p1_ans = part1_solution(&maps, &initial_seeds);
    println!("Part 1 answer: {}", p1_ans);

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
    let b = Instant::now();
    let p2_ans = part2_solution(&maps, &initial_seeds);
    println!("Part 2 answer: {:?}",b.elapsed());
    println!("Part 2 answer: {}",p2_ans);
}

fn part1_solution(maps: &[Vec<(u64, u64, u64)>], initial_seeds: &[u64]) -> u64 {
    let mut seeds = initial_seeds.to_owned();

    for map in maps.iter() {
        let mut mapped_seeds = seeds.clone();

        for &(dest, src, range) in map.iter().skip(1) {
            for (i, &seed) in seeds.iter().enumerate() {
                if seed >= src && seed < src + range {
                    mapped_seeds[i] = dest + (seed - src);
                }
            }
        }
        seeds = mapped_seeds;
    }

    *seeds.iter().min().unwrap()
}


// need fixing, very ugly rust code
// crabs are gonna hunt me down 
fn part2_solution(maps: &[Vec<(u64, u64, u64)>], initial_seeds: &[(u64, u64)]) -> u64 {
    let mut seeds = initial_seeds.to_owned();
    let mut mapped_results: Vec<(u64, u64)> = vec![];
    for map in maps {
        while !seeds.is_empty() {
            let mut in_range = false;
            let start = seeds[0].0;
            let offset = seeds[0].1;
            for mapping in map {
                let end = mapping.1 + mapping.2;
                if start < end  && start >= mapping.1 {
                    in_range = true;
                    // source range is entirely covered in range map
                    if start + offset < end {
                        mapped_results.push((start + mapping.0 - mapping.1, offset));
                        let _ = seeds.remove(0);
                        break;
                    }
                    else {
                        let new_offset = end - start;
                        mapped_results.push((start + mapping.0 - mapping.1,new_offset));
                        seeds.push((end, start + offset - end));
                        seeds.remove(0);
                        break;
                    }
                } 
                // source's start is less then range's beginning, end is inside range.
                else if start + offset < mapping.1 + mapping.2 && start + offset > mapping.1 {
                    in_range = true;
                    if start >= mapping.1 {
                        mapped_results.push((start + mapping.0 - mapping.1, offset));
                        seeds.remove(0);
                        break;
                    } else {
                        mapped_results.push((mapping.0, start + offset - mapping.1));
                        seeds.push((start, mapping.1 - start));
                        seeds.remove(0);
                        break;
                    }
                }
            }
            if !in_range {
                mapped_results.push((start, offset));
                seeds.remove(0);
            }
        }
        std::mem::swap(&mut seeds, &mut mapped_results);
    }
    seeds.iter().min_by(|&&a, &&b| a.0.cmp(&b.0)).unwrap().0
}