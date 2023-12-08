
pub fn part_1(input: &str) -> u64 {
    let arr: Vec<Vec<u64>> = input
        .lines()
        .into_iter()
        .map(|s| {
            s
                .find(":")
                .map(|i| &s[i+1..])
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<u64>>()
    }).collect();
    let mut total = 1;
    let (times, dsts) = (&arr[0], &arr[1]);
    for i in 0..times.len() {
        let start = searcher((0, times[i] / 2), dsts[i], times[i]);
        total *= (times[i] - start + 1) - start;
    }
    total
}

pub fn part_2(input: &str) -> u64 {
    let arr: Vec<u64> = input
        .lines()
        .into_iter()
        .map(|s| {
            let mut a = s
                .find(":")
                .map(|i| &s[i+1..])
                .unwrap()
                .to_string();
            a.retain(|c| !c.is_whitespace());
            a.parse::<u64>().unwrap()
        }).collect();
    let start = searcher((0, arr[0] / 2), arr[1], arr[0]);
    (arr[0] - start + 1) - start
}

fn searcher(range: (u64, u64), dst: u64, time: u64) -> u64 {
    if range.1 - range.0 <= 1 {
        range.1
    } else {
        let mid = (range.0 + range.1) / 2;
        if (time - mid) * mid < dst {
            searcher((mid, range.1), dst, time)
        } else {
            searcher((range.0, mid), dst, time)
        }
    }
}