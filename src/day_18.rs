
pub fn part_1(input: &str) -> i32 {
    let (mut y, mut x): (i32 ,i32) = (0, 0);
    input
        .lines()
        .into_iter()
        .fold(0, |acc, line| {
            let mut split: std::str::SplitWhitespace<'_> = line.split_whitespace();
            let dir = split.next().unwrap().chars().next().unwrap();
            let steps = split.next().unwrap().parse::<i32>().unwrap();
            let (y_prev, x_prev) = (y, x);
            match dir {
                'R' => x += steps,
                'L' => x -= steps,
                'U' => y -= steps,
                'D' => y += steps,
                _ => unreachable!(),
            }
            acc + (x + x_prev) * (y - y_prev) + steps
        }) / 2 + 1
}

pub fn part_2(input: &str) -> isize {
    let (mut y, mut x): (isize ,isize) = (0, 0);
    input
        .lines()
        .into_iter()
        .fold(0, |acc, line| {
            let hex = line.split_once("#").unwrap().1;
            let steps = isize::from_str_radix(&hex[0..hex.len() - 2], 16).unwrap();
            let (y_prev, x_prev) = (y, x);
            match hex.as_bytes()[hex.len() - 2] {
                b'0' => x += steps,
                b'1' => y += steps,
                b'2' => x -= steps,
                b'3' => y -= steps,
                _ => unreachable!(),
            }
            acc + (x + x_prev) * (y - y_prev) + steps
        }) / 2 + 1
}