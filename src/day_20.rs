// Flip-flop (%) - ignore high pulse, low pulse switch to opposite.
// Conjunction (&) - updates memory on input, if high pulse for all inputs, send low pulse, else high pulse.
// Broadcaster - sends same pulse received to all destination modules.

use std::collections::{HashMap, VecDeque};

trait Module: std::fmt::Debug {
    fn update(&mut self, high: bool, input: String);
    fn state(&self) -> bool;
    fn enter_key(&mut self, _: String) {}
    fn cont(&self, high: bool) -> bool { true }
}

#[derive(Debug)]
struct Broadcaster {
    state: bool,
}

impl Module for Broadcaster {
    fn update(&mut self, high: bool, _input: String) { self.state = high }

    fn state(&self) -> bool { self.state }
}

#[derive(Debug)]
struct FlipFlop {
    state: bool,
}

impl Module for FlipFlop {
    fn update(&mut self, high: bool, _input: String) {
        if !high {
            self.state = !self.state;
        }
    }

    fn state(&self) -> bool { self.state }

    fn cont(&self, high: bool) -> bool { !high }
}

#[derive(Debug)]
struct Conjunction {
    mem: HashMap<String, bool>,
    state: bool,
}

impl Module for Conjunction {
    fn update(&mut self, high: bool, input: String) {
        self.mem.insert(input, high);
        self.state = !self.mem.values().all(|p| *p);
    }

    fn state(&self) -> bool { self.state }

    fn enter_key(&mut self, key: String) {
        self.mem.insert(key, false);
    }
}

fn parse_system(input: &str) -> (HashMap<String, Box<dyn Module>>, HashMap<String, Vec<String>>) {
    let mut sys = HashMap::new();
    let mut dst = HashMap::new();
    input
        .lines()
        .for_each(|line| {
            let (name, destinations) = line.split_once(" -> ").unwrap();
            let destinations: Vec<String> = destinations
                .split(", ")
                .map(|s| s.to_string())
                .collect();
            let module: (String, Box<dyn Module>) = match name.chars().nth(0).unwrap() {
                'b' => (
                    "broadcaster".to_string(),
                    Box::new(Broadcaster { state: false }),
                ),
                '%' => (
                    name[1..].to_string(),
                    Box::new(FlipFlop {
                        state: false,
                    }),
                ),
                '&' => (
                    name[1..].to_string(),
                    Box::new(Conjunction {
                        state: false,
                        mem: HashMap::new(),
                    }),
                ),
                _ => unreachable!(),
            };
            sys.insert(module.0.clone(), module.1);
            dst.insert(module.0, destinations);
        });
    for (k, v) in dst.iter() {
        for m in v {
            sys.get_mut(m).map(|x| x.enter_key(k.clone()));
        }
    }
    (sys, dst)
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

pub fn part_1(input: &str) -> u32 {
    let (mut sys, dst) = parse_system(input);
    let mut queue = VecDeque::new();
    let (mut low, mut high) = (0, 0);
    for _ in 0..1000 {
        low += 1;
        queue.push_back(("broadcaster".to_string(), false));
        while let Some((name, input)) = queue.pop_front() {
            for dest in dst.get(&name).unwrap() {
                if input { high += 1 } else { low += 1 };
                if dest == "rx" || dest == "output" {
                    continue;
                }
                let n = sys.get_mut(dest).unwrap();
                if !n.cont(input) {
                    continue;
                }
                n.update(input, name.clone());
                queue.push_back((dest.to_string(), n.state()))
            }
        }
    }
    low * high
}

pub fn part_2(input: &str) -> usize {
    let (mut sys, dst) = parse_system(input);
    let mut cycle = vec![];
    for i in 0..4096 {
        let mut queue = VecDeque::new();
        queue.push_back(("broadcaster".to_string(), false));
        while let Some((name, input)) = queue.pop_front() {
            for dest in dst.get(&name).unwrap() {
                if dest == "rx" || dest == "output" || dest == "dg" {
                    if input {
                        cycle.push(i + 1);
                    }
                    continue;
                }
                let n = sys.get_mut(dest).unwrap();
                if !n.cont(input) {
                    continue;
                }
                n.update(input, name.clone());
                queue.push_back((dest.to_string(), n.state()))
            }
        }
    }
    lcm(&cycle)
}