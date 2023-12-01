use std::collections::HashMap;
use regex::Re

fn main() {
    println!("part 1 answer: {}", part_1());
    // println!("part 2 answer: {}", part_2())
}

fn part_1() -> u32 {
    std::fs::read_to_string("./inputs/1.txt")
        .unwrap()
        .lines()
        .into_iter()
        .map(|s| {
            let (mut first, mut last) = (None, None);
            for c in s.chars() {
                if c.is_ascii_digit() {
                    if first.is_none() {
                        first = c.to_digit(10);
                    }
                    last = c.to_digit(10);
                }
            }
            first.unwrap() * 10 + last.unwrap()
        }).sum()
}

const NUMS: [&'static str; 10] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];

fn part_2() -> i32 {
    std::fs::read_to_string("./inputs/1.txt")
        .unwrap()
        .lines()
        .into_iter()
        .map(|s| {
            

        }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
    }
}