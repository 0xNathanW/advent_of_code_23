
fn main() {
    println!("part 1 answer: {}", part_1());
    println!("part 2 answer: {}", part_2_v2())
}


fn part_1() -> u32 {
    std::fs::read_to_string("./inputs/1.txt")
        .unwrap()
        .lines()
        .map(|s| {
            let (mut first, mut last) = (None, None);
            for c in s.chars() {
                if c.is_ascii_digit() {
                    if first.is_none() {
                        first = c.to_digit(10);
                    }
                    last = c.to_digit(10);
                }
            }
            first.unwrap() * 10 + last.unwrap()
        }).sum()
}

fn str_to_num(s: &str) -> u32 {
    match s {
        "0" | "zero" => 0,
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        _ => 0, // Default case if none match
    }
}

// Doesn't work as doesn't match overlapping.
#[allow(dead_code)]
fn part_2() -> u32 {
    let re = regex::Regex::new(r"(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    std::fs::read_to_string("./inputs/1.txt")
        .unwrap()
        .lines()
        .map(|s| {
            dbg!(s);
            let mut matches = re.find_iter(s);
            let first = str_to_num(matches.next().unwrap().as_str());
            let last = if let Some(n) = matches.last() {
                str_to_num(n.as_str())
            } else {
                first
            };
            // println!("{} -> {}{}", s, a, b);
            first * 10 + last
        }).sum()
}

fn part_2_v2() -> u32 {
    let ac = aho_corasick::AhoCorasick::new(
        &[
            "1", "2", "3", "4", "5", "6", "7", "8", "9",
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ]
    ).unwrap();
    std::fs::read_to_string("./inputs/1.txt")
        .unwrap()
        .lines()
        .map(|s| {
            let mut matches = ac.find_overlapping_iter(s);
            let first = str_to_num(&s[matches.next().unwrap().range()]);
            let last = if let Some(m) = matches.last() {
                str_to_num(&s[m.range()])
            } else {
                first
            };
            first * 10 + last
        }).sum()
}