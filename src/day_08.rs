use std::collections::HashMap;


pub fn part_1(input: &str) -> u64 {

    let (dir_str, map_str) = input.split_once("\n").unwrap();
    let mut directions = dir_str.trim().chars();
    let map = get_mapping(map_str);
    let mut current_node = "AAA";
    let mut steps = 0;
    while current_node != "ZZZ" {
        match directions.next() {
            Some(c) => {
                steps += 1;
                current_node = match c {
                'L' => map.get(current_node).unwrap().0,
                'R' => map.get(current_node).unwrap().1,
                _ => unreachable!(),
                }
            },
            None => directions = dir_str.trim().chars(),
        }
    }
    steps
}

pub fn part_2(input: &str) -> u64 {
    let (dir_str, map_str) = input.split_once("\n").unwrap();
    let mut directions = dir_str.trim().chars();
    let map = get_mapping(map_str);
    let current_nodes: Vec<&str> = map
    .keys()
    .filter(|&k| k.ends_with('A'))
    .cloned()
    .collect();
    let mut steps = vec![0_u64; current_nodes.len()];
    let mut current_steps = 0;
    for (i, &(mut node)) in current_nodes.iter().enumerate() {
        while !node.ends_with('Z') {
            match directions.next() {
                Some(c) => {
                    current_steps += 1;
                    node = match c {
                        'L' => map.get(node).unwrap().0,
                        'R' => map.get(node).unwrap().1,
                        _ => unreachable!(),
                    }
                },
                None => directions = dir_str.trim().chars(),
            }
        }
        steps[i] = current_steps;
        current_steps = 0;
        directions = dir_str.trim().chars();
    }
    lcm(&steps)
}

fn get_mapping(map_str: &str) -> HashMap<&str, (&str, &str)> {
    map_str
        .lines()
        .into_iter()
        .skip(1)
        .map(|line| {
            let mapping = line
                .split_once("=")
                .unwrap();
            let mapping_to = mapping.1.trim()
                .trim_start_matches("(")
                .trim_end_matches(")")
                .split_once(",")
                .unwrap();
            (
                mapping.0.trim(),
                (mapping_to.0.trim(), mapping_to.1.trim())
            )
        }).collect()
}

pub fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}