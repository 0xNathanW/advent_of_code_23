use std::collections::HashSet;

pub fn part_1(path: &str) -> u32 {
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .into_iter()
        .map(|line|{
            let mut winners = HashSet::new();
            let mut n: u32 = 0;
            let nums: Vec<Vec<&str>> = line
                .find(":")
                .map(|idx| &line[idx+1..])
                .unwrap()
                .split("|")
                .into_iter()
                .map(|x| x.split_whitespace().collect())
                .collect();
            for n in &nums[0] {
                winners.insert(*n);
            }
            for m in &nums[1] {
                if let Some(_) = winners.get(*m) {
                    n += 1;
                }
            }
        if n != 0 { 
            2_u32.pow(n.saturating_sub(1))
        } else {
            0
        }
        }).sum()
}

pub fn part_2(path: &str) -> usize {
    let mut copies: Vec<usize> = vec![];
    std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .into_iter()
        .enumerate()
        .map(|(i, line)| {
            if i >= copies.len() {
                copies.push(1);
            }
            let mut winners = HashSet::new();
            let mut n: usize = 0;
            let nums: Vec<Vec<&str>> = line
                .find(":")
                .map(|idx| &line[idx+1..])
                .unwrap()
                .split("|")
                .into_iter()
                .map(|x| x.split_whitespace().collect())
                .collect();
            for n in &nums[0] {
                winners.insert(*n);
            }
            for m in &nums[1] {
                if let Some(_) = winners.get(*m) {
                    n += 1;
                }
            }
            for j in 1..(n + 1) {
                let idx = i + j;
                if idx >= copies.len() {
                    copies.push(1 + copies[i]);
                } else {
                    copies[i + j] += copies[i];
                }
            }
            
            copies[i]
        }).sum()
}