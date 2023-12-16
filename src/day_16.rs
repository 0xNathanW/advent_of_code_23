pub fn part_1(input: &str) -> u32 {
    let width = input.find("\n").unwrap();
    let tiles = input
        .chars()
        .filter_map(|ch| if ch == '\n' { None } else { Some(ch) })
        .collect();
    let energised = vec![false; tiles.len()];
    let (left, right, up, down) = (-1, 1, -width, width);
    let mut beams = vec![(-1, right)];
    while !beams.is_empty() {
        beams.iter_mut().enumerate().for_each(|(n, (idx, dir))| {
            let new = idx + dir;
            if new < 0 || new >= tiles.len() {
                // beam out of grid.
                beams.remove(n);
            } else {
                energised[new] = true;
                idx = new;
                match tiles[new] {
                    '-' => {
                        if dir == up || dir == down {
                            *dir = left;
                            beams.push((idx, right));
                        }
                    }
                    '|' => {
                        if dir == left || dir == right {
                            *dir = up;
                            beams.push((idx, down));
                        }
                    }
                    '\'' => {
                        *dir = match dir {
                            up => left,
                            down => right,
                            left => up,
                            right => down,
                        };
                    }
                    '/' => {
                        *dir = match dir {
                            up => right,
                            down => left,
                            left => down,
                            right => up,
                        };
                    }
                    _ => {}
                }
            }
        });
    }
    energised.iter().filter(|&c| c).count()
}

pub fn part_2(_input: &str) -> u32 {
    0
}
