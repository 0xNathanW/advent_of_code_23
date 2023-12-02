use std::collections::HashMap;


const NUM_RED: u32   = 12;
const NUM_GREEN: u32 = 13;
const NUM_BLUE: u32  = 14;

fn main() {
    println!("{}", part_2());
}

fn is_possible(x: Vec<&str>) -> bool {
    let target = match x[1] {
        "red" => NUM_RED,
        "green" => NUM_GREEN,
        "blue" => NUM_BLUE,
        _ => panic!()
    };
    x[0].parse::<u32>().unwrap() <= target
}

fn part_1() -> u32 {
    std::fs::read_to_string("./inputs/02.txt")
        .unwrap()
        .lines()
        .enumerate()
        .map(|(i, s)| {
            let mut possible = true;
            'outer: for pick in s
                .strip_prefix(&format!("Game {}: ", i + 1))
                .unwrap()
                .split(";") 
            {
                for combo in pick.trim().split(",") {
                    let a: Vec<&str> = combo.trim().split_whitespace().collect();
                    if !is_possible(a) {
                        possible = false;
                        break 'outer;
                    }
                }
            }
            if possible { i as u32 + 1 } else { 0 }
        }).sum()
}

fn part_2() -> u32 {
    std::fs::read_to_string("./inputs/02.txt")
        .unwrap()
        .lines()
        .enumerate()
        .map(|(i, s)| {
            let mut map: HashMap<&str, u32> = HashMap::new();
            map.insert("red", 0);
            map.insert("blue", 0);
            map.insert("green", 0);
            
            s.strip_prefix(&format!("Game {}: ", i + 1))
            .unwrap()
            .split(";")
            .into_iter()
            .for_each(|p| {
                p
                    .trim()
                    .split(",")
                    .into_iter()
                    .for_each(|c| {
                        let a: Vec<&str> = c.trim().split_whitespace().collect();
                        let n = a[0].parse::<u32>().unwrap();
                        if n > *map.get(a[1]).unwrap() {
                            map.insert(a[1], n);
                        }
                    });
            });
            map.get("red").unwrap() *
            map.get("blue").unwrap() *
            map.get("green").unwrap()
        }).sum()
}

