use std::collections::{HashSet, VecDeque};

enum Direction {
    North,
    South,
    East,
    West,
}

fn connect(pipe: char, direction: Direction) -> bool {
    match direction {
        Direction::North => pipe == '|' || pipe == 'L' || pipe == 'J',
        Direction::South => pipe == '|' || pipe == 'F' || pipe == '7',
        Direction::East => pipe == '-' || pipe == 'L' || pipe == 'F',
        Direction::West => pipe == '-' || pipe == 'J' || pipe == '7',
        
    }
}

fn initial_connects(arr: &Vec<Vec<char>>, start_node: (usize, usize)) -> Vec<(usize, usize)> {
    let mut connections = Vec::new();
    let (x, y) = start_node;
    if y < arr.len() - 1 && connect(arr[y + 1][x], Direction::North) {
        connections.push((x, y + 1));
    }
    if y > 0 && connect(arr[y - 1][x], Direction::South) {
        connections.push((x, y - 1));
    }
    if x > 0 && connect(arr[y][x - 1], Direction::East) {
        connections.push((x - 1, y));
    }
    if x < arr[0].len() - 1 && connect(arr[y][x + 1], Direction::West) {
        connections.push((x + 1, y));
    }
    connections
}

fn bfs(arr: &Vec<Vec<char>>, start: (usize, usize)) -> HashSet<(usize, usize)> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    visited.insert(start);
    for connection in initial_connects(arr, start) {
        queue.push_back(connection);
    }
    while let Some((x, y)) = queue.pop_front() {
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));
        let cell = arr[y][x];
        if connect(cell, Direction::North) && y > 0 {
            queue.push_back((x, y - 1));
        }
        if connect(cell, Direction::South) && y + 1 < arr.len() {
            queue.push_back((x, y + 1));
        }
        if connect(cell, Direction::East) && x + 1 < arr[0].len() {
            queue.push_back((x + 1, y));
        }
        if connect(cell, Direction::West) && x > 0 {
            queue.push_back((x - 1, y));
        }
    }
    visited
}

fn get_arr(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

pub fn part_1(input: &str) -> u32 {
    let arr = get_arr(input);
    let mut start_node = (0, 0);
    for (y, row) in arr.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'S' {
                start_node = (x, y);
            }
        }
    }
    bfs(&arr, start_node).len() as u32 / 2
}

fn num_enclosed_cells(
    visited: &HashSet<(usize, usize)>,
    row: &Vec<char>,
    range_x: (usize, usize),
    y: usize,
) -> u32 {
    let walls: HashSet<char> = ['L', 'J', '7', 'F', '|'].iter().cloned().collect();
    let corners: HashSet<char> = ['L', 'J', '7', 'F'].iter().cloned().collect();
    let mut count = 0;
    let mut inside = false;
    let mut prev_corner: Option<char> = None;
    for x in range_x.0..=range_x.1 {
        if visited.contains(&(x, y)) {
            let cell = row[x];
            if walls.contains(&cell) {
                if corners.contains(&cell) {
                    if let Some(prev) = prev_corner {
                        if (prev == 'L' && cell == '7') || (prev == 'F' && cell == 'J') {
                            prev_corner = None;
                            continue;
                        }
                    }
                    prev_corner = Some(cell);
                }
                inside = !inside;
            }
            continue;
        }
        if inside {
            count += 1;
        }
    }
    count
}

pub fn part_2(input: &str) -> u32 {
    let arr = get_arr(input);
    let mut start_node = (0, 0);
    for (y, row) in arr.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'S' {
                start_node = (x, y);
            }
        }
    }
    let visited = bfs(&arr, start_node);
    let (min_x, max_x, min_y, max_y) = visited
        .iter()
        .fold((usize::MAX, 0, usize::MAX, 0), |(min_x, max_x, min_y, max_y), (x, y)| {
            (min_x.min(*x), max_x.max(*x), min_y.min(*y), max_y.max(*y))
        });
    (min_y..=max_y)
        .into_iter()
        .map(|y| num_enclosed_cells(&visited, &arr[y], (min_x, max_x), y))
        .sum()
}
