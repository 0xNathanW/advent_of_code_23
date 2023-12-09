
pub fn part_1(input: &str) -> i32 {
    let seqs = get_seqs(input);
    let mut total = 0;
    for seq in seqs {
        let mut last_elems = vec![seq[seq.len() - 1]];
        let mut current = seq;
        loop {
            let mut diffs = vec![0; current.len() - 1];
            let mut all_same = true;
            for i in 0..diffs.len() {
                diffs[i] = current[i + 1] - current[i];
                if diffs[i] != diffs[0] { all_same = false; }
            }
            if all_same {
                total += last_elems
                    .iter()
                    .rev()
                    .fold(diffs[diffs.len() - 1], |acc, n| acc + n);
                break;
            }
            last_elems.push(diffs[diffs.len() - 1]);
            current = diffs;
        }
    }
    total
}

pub fn part_2(input: &str) -> i32 {
    let seqs = get_seqs(input);
    let mut total = 0;
    for seq in seqs {
        let mut first_elems = vec![seq[0]];
        let mut current = seq;
        loop {
            let mut diffs = vec![0; current.len() - 1];
            let mut all_same = true;
            for i in 0..diffs.len() {
                diffs[i] = current[i + 1] - current[i];
                if diffs[i] != diffs[0] { all_same = false; }
            }
            if all_same {
                total += first_elems
                .iter()
                .rev()
                .fold(diffs[diffs.len() - 1], |acc, n| - acc + n);
                break;
            }
            first_elems.push(diffs[0]);
            current = diffs;
        }
    }
    total
}

fn get_seqs(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|s| {
            s
                .trim()
                .split_ascii_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect()
        }).collect()
}

