use std::collections::HashMap;

pub fn part_1(input: &str) -> u32 {
    input.trim().split(",").into_iter().map(hash).sum()
}

pub fn part_2(input: &str) -> u32 {
    let mut boxes: HashMap<u32, Vec<(&str, u32)>> = HashMap::new();
    input.trim().split(",").for_each(|s| {
        if let Some((label, focal_len)) = s.split_once("=") {
            let f = focal_len.parse::<u32>().unwrap_or(0);
            let boxx = boxes.entry(hash(label)).or_insert_with(Vec::new);
            if let Some(lens) = boxx.iter_mut().find(|(l, _)| *l == label) {
                lens.1 = f;
            } else {
                boxx.push((label, f));
            }
        } else {
            let label = &s[..s.len() - 1];
            if let Some(boxx) = boxes.get_mut(&hash(label)) {
                if let Some(pos) = boxx.iter().position(|(l, _)| *l == label) {
                    boxx.remove(pos);
                }
            }
        }
    });
    boxes.into_iter()
        .flat_map(|(b, boxx)| {
            boxx
                .into_iter()
                .enumerate()
                .map(move |(slot, (_, focal_len))| (b as u32 + 1) * (slot as u32 + 1) * focal_len)
        })
        .sum()
}

fn hash(input: &str) -> u32 {
    input.chars().fold(0, |acc, ch| ((acc + ch as u32) * 17) % 256)
}