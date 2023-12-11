use std::collections::HashSet;

pub fn part_1(input: &str) -> usize {
    solve(input, false)
}

pub fn part_2(input: &str) -> usize {
    solve(input, true)
}

fn solve(input: &str, part_2: bool) -> usize {
    let mut galaxies = vec![];
    let mut empty_rows = HashSet::new();
    let mut empty_cols = HashSet::new();
    let (mut max_x, mut max_y) = (0, 0);
    input
        .lines()
        .enumerate()
        .into_iter()
        .for_each(|(y, row)| {
            for (x, c) in row.chars().enumerate() {
                if c != '.' {
                    galaxies.push((x, y));
                    max_x = max_x.max(x);
                    max_y = max_y.max(y);
                    empty_rows.insert(y);
                    empty_cols.insert(x);
                }
            }
        });
    // Invert Sets.
    (0..=max_x).into_iter().for_each(|x| invert_set(&mut empty_cols, x));
    (0..=max_y).into_iter().for_each(|y| invert_set(&mut empty_rows, y));
    let mut total = 0;
    let mut paths_done = HashSet::new();

    for &a in galaxies.iter() {
        for &b in galaxies.iter() {
            let pair = order(a, b);
            if paths_done.contains(&pair) || a == b{
                continue;
            }
            let dist = manhattan_dist(pair.0, pair.1, &empty_rows, &empty_cols, part_2);
            total += dist;
            paths_done.insert(pair);
        }
    }
    total
}

fn order(a: (usize, usize), b: (usize, usize)) -> ((usize, usize), (usize, usize)) {
    if a < b { (a, b) } else { (b, a) }
}

fn invert_set(set: &mut HashSet<usize>, n: usize) {
    if set.contains(&n) {
        set.remove(&n);
    } else {
        set.insert(n);
    }
}

fn manhattan_dist(a: (usize, usize), b: (usize, usize), rows: &HashSet<usize>, cols: &HashSet<usize>, part_2: bool) -> usize {
    let offset = if part_2 { 1000000 } else { 2 };
    let mut distance = 0;
    let (mut x_min, x_max) = if a.0 <= b.0 { (a.0, b.0) } else { (b.0, a.0) };
    while x_min < x_max {
        distance += if cols.contains(&x_min) { offset } else { 1 };
        x_min += 1;
    }
    let (mut y_min, y_max) = if a.1 <= b.1 { (a.1, b.1) } else { (b.1, a.1) };
    while y_min < y_max {
        distance += if rows.contains(&y_min) { offset } else { 1 };
        y_min += 1;
    }
    distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrong() {
        let mut empty_rows = HashSet::new();
        let mut empty_cols = HashSet::new();
        empty_rows.insert(7);
        empty_rows.insert(3);
        empty_cols.insert(2);
        empty_cols.insert(5);
        empty_cols.insert(8);
        let d = manhattan_dist((3, 0), (2, 0), &empty_rows, &empty_cols, false);
        dbg!(d);
    }
}