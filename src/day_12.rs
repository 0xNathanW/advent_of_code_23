use std::collections::HashMap;

pub fn part_1(input: &str) -> usize {
    parse_input(input)
        .into_iter()
        .map(|(springs, groups)| count_arrangements(
            &springs, 
            &groups,
            0, 
            0, 
            0, 
            &mut HashMap::new()
        ))
        .sum()
}

pub fn part_2(input: &str) -> usize {
    parse_input(input)
        .into_iter()
        .map(|(springs, groups)| count_arrangements(
            &fold_spring(&springs), 
            &groups.repeat(5),
            0, 
            0, 
            0, 
            &mut HashMap::new()
        ))
        .sum()
}

fn count_arrangements(
    springs: &[char], 
    blocks: &[usize], 
    s: usize, // spings idx
    b: usize, // blocks idx
    current: usize, 
    dp: &mut HashMap<(usize, usize, usize), usize>
) -> usize {
    let key = (s, b, current);
    if let Some(&cached) = dp.get(&key) {
        return cached;
    }
    if s == springs.len() {
        return if b == blocks.len() && current == 0 { 
                    1 
                } else if b == blocks.len() - 1 && blocks[b] == current {
                    1 
                } else { 0 };
    }
    let mut total = 0;
    for &c in ['.', '#'].iter() {
        if springs[s] == c || springs[s] == '?' {
            if c == '.' && current == 0 {
                total += count_arrangements(springs, blocks, s + 1, b, 0, dp);
            } else if c == '.' && current > 0 && b < blocks.len() && blocks[b] == current {
                total += count_arrangements(springs, blocks, s + 1, b + 1, 0, dp);
            } else if c == '#' {
                total += count_arrangements(springs, blocks, s + 1, b, current + 1, dp);
            }
        }
    }
    dp.insert(key, total);
    total
}

fn fold_spring(s: &[char]) -> Vec<char> {
    s.iter()
     .copied()
     .cycle()
     .take(s.len() * 5) // Take only the required number of characters
     .enumerate()
     .flat_map(|(i, c)| {
         if i % s.len() == s.len() - 1 && i < s.len() * 5 - 1 {
             vec![c, '?'] // Insert separator after each repetition except the last
         } else {
             vec![c]
         }
     })
     .collect()
}

fn parse_input(input: &str) -> Vec<(Vec<char>, Vec<usize>)> {
    input
        .lines()
        .into_iter()
        .map(|line| {
            let split = line.split_once(" ").unwrap();
            (
                split.0.chars().collect(),
                split.1
                    .split(",")
                    .into_iter()
                    .map(|s| s.parse().unwrap())
                    .collect()
            )
        }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeat() {
        let s = "???.###".chars().collect::<Vec<char>>();
        let out = fold_spring(&s).iter().collect::<String>();
        dbg!(out);
    }
}