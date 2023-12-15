use std::collections::HashMap;

pub fn part_1(input: &str) -> usize {
    let arr: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    calculate_load(&arr)
}

pub fn part_2(input: &str) -> usize {
    let mut arr: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut cache = HashMap::new();
    let mut n = 0;
    let (mut cycles, mut cycle_len) = (0, 0);
    loop {
        n += 1;
        cycle(&mut arr);
        if let Some(dup) = cache.insert(arr_to_string(&arr), n) {
            cycles = n;
            cycle_len = n - dup;
            break;
        }
    }
    let cycles_left = (1_000_000_000 - cycles) % cycle_len;
    for _ in 0..cycles_left {
        cycle(&mut arr);
    }

    calculate_load(&arr)
}

fn calculate_load(arr: &Vec<Vec<char>>) -> usize {
    let mut total = 0;
    let mut blocks: Vec<Option<(usize, usize)>> = vec![None; arr[0].len()];
    arr.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, ch)| match ch {
            '#' => blocks[j] = None,
            'O' => {
                if let Some(free_row) = blocks[j] {
                    total += arr.len() - free_row.0;
                    blocks[j] = Some((free_row.0 + 1, free_row.1 + 1));
                } else {
                    total += arr.len() - i;
                }
            }
            '.' => blocks[j] = blocks[j].map_or(Some((i, 0)), |val| Some((val.0, val.1 + 1))),
            _ => unreachable!(),
        });
    });
    total
}

fn roll(arr: &mut Vec<Vec<char>>) {
    for col in 0..arr[0].len() {
        let mut next = 0;
        for row in 0..arr.len() {
            match arr[row][col] {
                'O' => {
                    if next != row {
                        arr[next][col] = 'O';
                        arr[row][col] = '.';
                    }
                    next += 1;
                }
                '#' => {
                    next = row + 1;
                }
                _ => {}
            }
        }
    }
}

fn rotate_90(arr: &mut Vec<Vec<char>>) {
    for i in 0..arr.len() {
        for j in i..arr[i].len() {
            let tmp = arr[i][j];
            arr[i][j] = arr[j][i];
            arr[j][i] = tmp;
        }
    }
}

fn cycle(arr: &mut Vec<Vec<char>>) {
    for _ in 0..4 {
        roll(arr);
        rotate_90(arr);
    }
}

fn arr_to_string(arr: &Vec<Vec<char>>) -> String {
    arr.iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}
