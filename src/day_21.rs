use std::collections::{HashSet, VecDeque};

pub fn part_1(input: &str, target: u32) -> u32 {
    let (src, grid) = get_grid(input);
    let mut queue = VecDeque::new();
    let mut visited_step = HashSet::new();
    let size = (grid.len(), grid[0].len());
    adjacent_plots(src, size)
        .into_iter()
        .for_each(|plot| queue.push_back((plot, 1)));
    let mut last_step = 0;
    while let Some(((y, x), steps)) = queue.pop_front() {
        if grid[y][x] == '#' || visited_step.contains(&(y, x)) {
            continue;
        }
        if steps != last_step {
            if steps == target + 1 {
                break;
            } else {
                last_step = steps;
                visited_step.clear();
            }
        }
        visited_step.insert((y, x));
        adjacent_plots((y, x), size)
            .into_iter()
            .for_each(|plot| queue.push_back((plot, steps + 1)));
    }
    visited_step.len() as u32
}

fn get_grid(input: &str) -> ((usize, usize), Vec<Vec<char>>) {
    let mut src = (0, 0);
    let grid: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, ch)| {
                    if ch == 'S' {
                        src = (y, x)
                    };
                    ch
                })
                .collect()
        })
        .collect();
    (src, grid)
}

fn adjacent_plots(plot: (usize, usize), size: (usize, usize)) -> Vec<(usize, usize)> {
    let mut plots = vec![];
    if plot.0 > 0 {
        plots.push((plot.0 - 1, plot.1))
    }
    if plot.0 < size.0 {
        plots.push((plot.0 + 1, plot.1))
    }
    if plot.1 > 0 {
        plots.push((plot.0, plot.1 - 1))
    }
    if plot.1 < size.1 {
        plots.push((plot.0, plot.1 + 1))
    }
    plots
}

// Part 2 i have literally no idea.