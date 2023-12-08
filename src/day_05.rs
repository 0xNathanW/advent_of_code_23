
pub fn part_1(input: &str) -> usize {
    let arr = get_arr(input);
    let mut lowest = usize::MAX;
    for seed in arr[0][0].iter() {
        let mut idx = *seed;
        for map in arr[1..].iter() {
            for range in map {
                if idx >= range[1] && idx <= range[1] + range[2] {
                    idx = range[0] + idx - range[1];
                    break;
                }
            }
        }
        lowest = lowest.min(idx);
    }
    lowest
}

// Yeh I brute forced it https://media.tenor.com/epNMHGvRyHcAAAAC/gigachad-chad.gif
pub fn part_2(path: &str) -> usize {
    let arr = get_arr(path);
    let mut lowest = usize::MAX;
    let seed_ranges: Vec<(usize, usize)> = arr[0][0]
        .chunks_exact(2)
        .map(|v| (v[0], v[0] + v[1]))
        .collect();
    let ranges_len = seed_ranges.len();
    for (i, (start, finish)) in seed_ranges.into_iter().enumerate() {
        for seed in start..finish {
            let mut idx = seed;
            for map in arr[1..].iter() {
                for range in map {
                    if idx >= range[1] && idx <= range[1] + range[2] {
                        idx = range[0] + idx - range[1];
                        break;
                    }
                }
            }
            lowest = lowest.min(idx);
        }
        println!("range {} of {} done {}..{}", i+1, ranges_len, start, finish);
    }

    lowest
}

fn get_arr(input: &str) -> Vec<Vec<Vec<usize>>> {
    input
        .split("\r\n\r")
        .into_iter()
        .map(|map|{
            map
                .find(":")
                .map(|i| &map[i+1..])
                .unwrap()
                .split("\n")
                .filter(|&s| {
                    !s.is_empty()
                })
                .into_iter()
                .map(|row| {
                    row
                        .trim()
                        .split_whitespace()
                        .into_iter()
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                }).filter(|v| !v.is_empty())
                .collect()
        }).collect()
}
