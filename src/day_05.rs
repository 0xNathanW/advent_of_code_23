use std::collections::HashSet;
use std::sync::Arc;
use std::thread;

pub fn part_1(path: &str) -> usize {
    let arr = get_arr(path);
    let mut lowest = usize::MAX;
    for seed in arr[0][0].iter() {
        let mut idx = *seed;
        for map in arr[1..].iter() {
            for range in map {
                if idx >= range[1] && idx <= range[1] + range[2] {
                    idx = range[0] + (idx - range[1]);
                    break;
                }
            }
        }
        lowest = lowest.min(idx);
    }
    lowest
}

// Too tired so brute force with threads lol.
// Pretty sure optimal is some sort of bottom up memorisation with hashmap or smth.
pub fn part_2(path: &str) -> usize {
    let arr = get_arr(path);
    let ranges = merge_ranges(&arr[0][0]);
    let mut maps_v = vec![];
    for map in &arr {
        let mut mappings = vec![];
        for range in map {
            mappings.push(Mapping {
                start: range[1],
                end: range[1] + range[0] - 1,
                offset: range[0] - range[1],
            });
        }
        mappings.sort_by(|a, b| a.start.cmp(&b.start));
        maps_v.push(mappings);
    }
    drop(arr);
    let mut lowest = usize::MAX;
    'range_loop: for range in ranges.iter() {
        'map_loop: for map in maps_v.iter() {

            'mapping_loop: for mapping in map {


            }

        }
    }
    0
}

struct Mapping {
    start:  usize,
    end:    usize,
    offset: usize,
}

fn merge_ranges(ranges: &Vec<usize>) -> Vec<(usize, usize)> {
    let mut v: Vec<(usize, usize)> = vec![];
    for i in (0..ranges.len()).step_by(2) {
        v.push((ranges[i], ranges[i] + ranges[i + 1]));
    }
    v.sort_by(|a, b| a.0.cmp(&b.0));
    let mut merged = vec![];
    let mut current = v[0].clone();

    for r in v.iter().skip(1) {
        if r.0 <= current.1 {
            current.1 = current.1.max(r.1);
        } else {
            merged.push(current);
            current = r.clone();
        }
    }
    merged.push(current);
    merged
}

fn get_arr(path: &str) -> Vec<Vec<Vec<usize>>> {
    std::fs::read_to_string(path)
    .unwrap()
    .split("\n\n")
    .into_iter()
    .map(|map|{
        map
            .find(":")
            .map(|i| &map[i+1..])
            .unwrap()
            .split("\n")
            .filter(|&s| !s.is_empty())
            .into_iter()
            .map(|row| {
                row
                    .trim()
                    .split_whitespace()
                    .into_iter()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>()
            }).collect()
    }).collect()
}