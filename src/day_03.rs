pub fn part_1(path: &str) -> u32 {
    let mut total = 0_u32;
    let mut num: (String, usize) = Default::default();
    let content: Vec<Vec<char>> = std::fs::read_to_string(path).unwrap()
        .lines()
        .into_iter()
        .map(|s| s.chars().collect())
        .collect();

    for (i, row) in content.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            match c {
                '0'..='9' => {
                    if num.0.is_empty() {
                        num.1 = j;
                    }
                    num.0.push(*c);
                },
                _ => {
                    if !num.0.is_empty() {
                        // Check if number has any adjacent symbols.
                        if is_adjacent_to_symbol(num.1, j - 1, i, &content) {
                            total += num.0.parse::<u32>().unwrap();
                        }
                        num = Default::default();
                    }
                }
            }

        }
        // May still ahve number in buffer.
        if !num.0.is_empty() {
            if is_adjacent_to_symbol(num.1, num.1 + num.0.len() - 1, i, &content) {
                total += num.0.parse::<u32>().unwrap();
            }
        }
        num = Default::default();
    }
    total
}

pub fn is_adjacent_to_symbol(start_idx: usize, end_idx: usize, row_idx: usize, arr: &Vec<Vec<char>>) -> bool {
    for row in row_idx.saturating_sub(1)..=(row_idx + 1).clamp(0, arr.len()) {
        for col in start_idx.saturating_sub(1)..=(end_idx + 1).clamp(0, arr[0].len() - 1) {
            if row == row_idx && start_idx <= col && col <= end_idx {
                continue;
            }
            if arr[row][col] != '.' {
                return true;
            }
        }
    }
    false
}

pub fn part_2(path: &str) -> u32 {
    let content: Vec<Vec<char>> = std::fs::read_to_string(path).unwrap()
        .lines()
        .into_iter()
        .map(|s| s.chars().collect())
        .collect();
    let row_size = content[0].len();
    let (mut count, mut product, mut total) = (0, 1, 0);
    let mut invalid_idx = vec![];
    let mut buf = String::new();
    for (i, row) in content.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c != '*' {
                continue;
            }
            'outer: for row in i.saturating_sub(1)..=(i + 1).clamp(0, content.len()) {
                for col in j.saturating_sub(1)..=(j + 1).clamp(0, row_size) {
                    if content[row][col].is_digit(10) && !invalid_idx.contains(&col) {
                        let mut x = col;
                        while x > 0 && content[row][x - 1].is_digit(10) {
                            x -= 1;
                        }
                        while x < row_size && content[row][x].is_digit(10) && !invalid_idx.contains(&x) {
                            buf.push(content[row][x]);
                            invalid_idx.push(x);
                            x += 1;
                        }
                        product *= buf.parse::<u32>().unwrap();
                        count += 1;
                        buf.clear();

                        if count == 2 {
                            break 'outer;
                        }
                    }
                }
                invalid_idx.clear();
            }
            if count == 2 {
                total += product;
            }
            product = 1;
            count = 0;
            invalid_idx.clear()
            
        }
    }
    total
}