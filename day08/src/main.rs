mod util;
use core::panic;
use std::{collections::HashMap, fs::File, io::Read};

fn main() {
    let mut text = String::new();
    let res = File::open("input.txt")
        .expect("Error: Input file not found")
        .read_to_string(&mut text)
        .unwrap();
    assert!(res > 0, "Error: Input file is empty");

    let path_to_follow = text
        .split('\n')
        .next()
        .expect("Couldn't read file contents (first line).").as_bytes();

    //skip first line and empty line
    let paths: Vec<&str> = text.split('\n').skip(2).collect();

    //hashmap cuz it be going fast asf.
    let mut nodes_map: HashMap<&str, [&str; 2]> = HashMap::new();
    for path in paths {
        let starting_node = path.split(" =").next().unwrap();
        let left = path.split(|ch| ch == '(' || ch == ',').nth(1).unwrap();
        let right = path.split(", ").nth(1).unwrap();
        nodes_map.insert(starting_node, [left, &right[..right.len() - 1]]);
    }
    println!("{}", find_paths_p1(path_to_follow, &nodes_map));
    println!("{}", find_paths_p2(path_to_follow, &nodes_map));
}

fn find_paths_p1(path_to_follow: &[u8], nodes_map: &HashMap<&str, [&str; 2]>) -> usize {
    let mut count = 0;
    let mut current_path = "AAA";

    // go through the paths until we hit ZZZ,
    // if left, get the value's first(index 0) element
    // get the second(index 1) element if right 
    // and increase count in each iteration,
    // else panic!
    while current_path != "ZZZ" {
        current_path = match path_to_follow[count % path_to_follow.len()] {
            b'L' => nodes_map.get(&current_path).unwrap()[0],
            b'R' => nodes_map.get(&current_path).unwrap()[1],
            _ => panic!("Wrong direction, only L and R are allowed.")
        };
        count += 1;
    }
    count
}

fn find_paths_p2(path_to_follow: &[u8], nodes_map: &HashMap<&str, [&str; 2]>) -> usize {

    //get the keys that end with the letter "A"
    let start_nodes = nodes_map
        .keys()
        .filter(|key| key.ends_with('A'))
        .cloned()
        .collect::<Vec<_>>();
    // find how many iterations it takes for each
    // starting node to get to a node that ends with Z,
    // and find the least common multiple of all those path lengths
    let mut node_ends = vec![];
    for node in start_nodes {
        let mut node = node;
        let mut count = 0;
        while !node.ends_with('Z') {
            match path_to_follow[count % path_to_follow.len()] {
                b'L' => {
                    node = nodes_map.get(&node).unwrap()[0];
                }
                b'R' => {
                    node = nodes_map.get(&node).unwrap()[1];
                }
                _ => panic!("Wrong direction, only L and R are allowed."),
            }
            count += 1;
        }
        node_ends.push(count);
    }
    //return total least common multiple of the nodes
    util::total_lcm(&node_ends)
}

