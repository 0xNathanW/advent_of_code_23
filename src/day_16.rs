type Beam = ((usize, usize), Direction);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn flag(self) -> u8 {
        match self {
            Direction::Up    => 0b0001,
            Direction::Down  => 0b0010,
            Direction::Left  => 0b0100,
            Direction::Right => 0b1000,
        }
    }
}

enum Mirror {
    Vertical,
    Horizontal,
    AngledForward,
    AngledBackward,
    None,
}

impl From<char> for Mirror {
    fn from(value: char) -> Self {
        match value {
            '|' => Mirror::Vertical,
            '-' => Mirror::Horizontal,
            '/' => Mirror::AngledForward,
            '\\' => Mirror::AngledBackward,
            '.' => Mirror::None,
            _ => unreachable!(),
        }
    }
}

fn get_tiles(input: &str) -> Vec<Vec<(u8, Mirror)>> {
    input
        .lines()
        .map(|line| line
            .chars()
            .map(|ch| (0, ch.into()))
            .collect())
        .collect()
}

fn calculate_energised(tiles: &mut Vec<Vec<(u8, Mirror)>>, src: Beam) -> u32 {
    let size = (tiles.len(), tiles[0].len());
    let mut beams = vec![src];
    while let Some(beam) = beams.pop() {
        let ((y, x), dir) = beam;
        let (tile_flag, mirror) = &mut tiles[y][x];
        if *tile_flag & dir.flag() != 0 {
            continue;
        }
        *tile_flag |= dir.flag();
        beams.extend(next_tile_mirror(beam, mirror, size).into_iter().flatten())
    }
    tiles.iter_mut().flatten().fold(0, |acc, (f, _)| {
        if *f == 0{
            acc
        } else {
            *f = 0;
            acc + 1
        }
    })
}

fn next_tile(point: (usize, usize), dir: Direction, size: (usize, usize)) -> Option<Beam> {
    let (y, x) = point;
    match dir {
        Direction::Up => Some(((y.checked_sub(1)?, x), dir)),
        Direction::Down => Some((((y < size.0 - 1).then_some(y + 1)?, x), dir)),
        Direction::Left => Some(((y, x.checked_sub(1)?), dir)),
        Direction::Right => Some(((y, (x < size.1 - 1).then_some(x + 1)?), dir)),
    }
} 

fn next_tile_mirror(beam: Beam, mirror: &Mirror, size: (usize, usize)) -> [Option<Beam>; 2] {
    let dir = beam.1;
    let point = beam.0;
    match mirror {
        Mirror::Vertical => {
            match dir {
                Direction::Left | Direction::Right => {
                    [next_tile(point, Direction::Up, size), next_tile(point, Direction::Down, size)]
                },
                _ => [next_tile(point, dir, size), None]
            }
        },
        Mirror::Horizontal => {
            match dir {
                Direction::Up | Direction::Down => {
                    [next_tile(beam.0, Direction::Left, size), next_tile(point, Direction::Right, size)]
                },
                _ => [next_tile(point, dir, size), None],
            }
        },
        Mirror::AngledForward => {
            match dir {
                Direction::Up => [next_tile(point, Direction::Right, size), None],
                Direction::Down => [next_tile(point, Direction::Left, size), None],
                Direction::Left => [next_tile(point, Direction::Down, size), None],
                Direction::Right => [next_tile(point, Direction::Up, size), None],
            }
        },
        Mirror::AngledBackward => {
            match dir {
                Direction::Up => [next_tile(point, Direction::Left, size), None],
                Direction::Down => [next_tile(point, Direction::Right, size), None],
                Direction::Left => [next_tile(point, Direction::Up, size), None],
                Direction::Right => [next_tile(point, Direction::Down, size), None],
            }
        },
        Mirror::None => [next_tile(point, dir, size), None],
    }
}

pub fn part_1(input: &str) -> u32 {
    let mut tiles = get_tiles(input);
    calculate_energised(&mut tiles, ((0, 0), Direction::Right))
}

pub fn part_2(input: &str) -> u32 {
    let mut tiles = get_tiles(input);
    let mut max = 0;
    let size = (tiles.len(), tiles[0].len());    
    // Col 0 rows.
    for row in 0..size.0 {
        max = max.max(calculate_energised(&mut tiles, ((row, 0), Direction::Right)));
        max = max.max(calculate_energised(&mut tiles, ((row, size.1 - 1), Direction::Left)));
    }
    for col in 0..size.1 {
        max = max.max(calculate_energised(&mut tiles, ((0, col), Direction::Down)));
        max = max.max(calculate_energised(&mut tiles, ((size.0 - 1, col), Direction::Down)));
        
    }
    max
}
