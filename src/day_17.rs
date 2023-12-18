use std::collections::{BinaryHeap, HashMap};


pub fn part_1(input: &str) -> u32 {
    dijkstras(input, 1, 3)
}

pub fn part_2(input: &str) -> u32 {
    dijkstras(input, 4, 10)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Node {
    cost: u32,
    coords: (usize, usize),
    direction: Direction,
    steps: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost).then_with(|| self.coords.cmp(&other.coords))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct NodeKey {
    coords: (usize, usize),
    direction: Direction,
    steps: u32,
}

impl From<Node> for NodeKey {
    fn from(value: Node) -> Self {
        NodeKey {
            coords: value.coords,
            direction: value.direction,
            steps: value.steps,
        }
    }
}

fn dijkstras(input: &str, min_step: u32, max_step: u32) -> u32 {
    let city: Vec<Vec<u32>> = input
        .lines()
        .into_iter()
        .map(|line| line
            .chars()
            .map(|ch| ch.to_digit(10).unwrap())
            .collect())
        .collect();
    let size = (city.len(), city[0].len());
    let (src, dst) = ((0_usize, 0_usize), (size.0 - 1, size.1 - 1));
    // Shortest distance from src to node => dists[node]
    let mut dists: HashMap<NodeKey, u32> = HashMap::new();
    let a = Node { cost: 0, coords: src, direction: Direction::Right, steps: 0 };
    let b = Node { cost: 0, coords: src, direction: Direction::Right, steps: 0 };
    dists.insert(a.into(), 0);
    dists.insert(b.into(), 0);
    let mut q = BinaryHeap::new();
    q.push(a);
    q.push(b);

    while let Some(node @ Node { cost, coords, direction, steps }) = q.pop() {
        if coords == dst && steps >= min_step {
            return cost;
        }
        if dists.get(&node.into()).is_some_and(|&c| c < cost) {
            continue;
        }

        for (p, d) in adjacent(coords, direction, size) {
            let nxt_node = Node {
                cost: cost + city[p.0][p.1],
                coords: p,
                direction: d,
                steps: if d == direction { steps + 1 } else { 1 },
            };
            if nxt_node.steps > max_step || dists.get(&nxt_node.into()).is_some_and(|&c| c <= nxt_node.cost) {
                continue;
            }
            if nxt_node.direction != direction && steps < min_step {
                continue;
            }
            q.push(nxt_node);
            dists.insert(nxt_node.into(), nxt_node.cost);
        }

    }
    u32::MAX
}

fn adjacent(coords: (usize, usize), dir: Direction, size: (usize, usize)) -> Vec<((usize, usize), Direction)> {
    [Direction::Up, Direction::Down, Direction::Left, Direction::Right]
        .into_iter()
        .filter(|d| *d != dir.opposite())
        .filter_map(|d| next_node(coords, d, size))
        .collect()
}

fn next_node(point: (usize, usize), dir: Direction, size: (usize, usize)) -> Option<((usize, usize), Direction)> {
    let (y, x) = point;
    match dir {
        Direction::Up => Some(((y.checked_sub(1)?, x), dir)),
        Direction::Down => Some((((y < size.0 - 1).then_some(y + 1)?, x), dir)),
        Direction::Left => Some(((y, x.checked_sub(1)?), dir)),
        Direction::Right => Some(((y, (x < size.1 - 1).then_some(x + 1)?), dir)),
    }
} 