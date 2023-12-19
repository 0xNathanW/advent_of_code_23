use std::collections::HashMap;

#[derive(Debug)]

struct Rule<'a> {
    condition: Option<Condition>,
    destination: &'a str,
}

#[derive(Debug)]
struct Condition {
    category: Category,
    greater_than: bool,
    target: u64,
}

#[derive(Clone, Copy, Debug)]
enum Category {
    X,
    M,
    A,
    S,
}

impl From<char> for Category {
    fn from(value: char) -> Self {
        match value {
            'a' => Category::A,
            'm' => Category::M,
            'x' => Category::X,
            's' => Category::S,
            _ => unreachable!(),
        }
    }
}

fn get_workflows(input: &str) -> HashMap<&str, Vec<Rule>> {
    input
        .lines()
        .into_iter()
        .map(|line| {
            let (name, flow) = line.split_once("{").unwrap();
            let rules = flow
                .trim_end_matches("}")
                .split(",")
                .map(|rule| {
                    rule.split_once(":").map_or(
                        Rule {
                            condition: None,
                            destination: rule,
                        },
                        |(c, d)| {
                            let mut c_iter = c.chars();
                            let category = c_iter.next().unwrap().into();
                            let greater_than = if c_iter.next().unwrap() == '>' {
                                true
                            } else {
                                false
                            };
                            let target = c_iter
                                .into_iter()
                                .collect::<String>()
                                .parse::<u64>()
                                .unwrap();
                            Rule {
                                condition: Some(Condition {
                                    category,
                                    greater_than,
                                    target,
                                }),
                                destination: d,
                            }
                        },
                    )
                })
                .collect();
            (name, rules)
        })
        .collect()
}


pub fn part_1(input: &str) -> u64 {
    let (workflows_raw, ratings) = input.split_once("\n\n").unwrap();
    let workflows = get_workflows(workflows_raw);
    ratings
        .lines()
        .into_iter()
        .map(|line| {
            let rating = &line[1..line.len() - 1];
            let ratings: Vec<u64> = rating
                .split(",")
                .into_iter()
                .map(|r| r.split_once("=").unwrap().1.parse::<u64>().unwrap())
                .collect();
            let mut current_workflow = "in";
            while current_workflow != "A" && current_workflow != "R" {
                let rules = workflows
                    .get(current_workflow)
                    .expect("could not find workflow");
                for rule in rules {
                    if let Some(con) = &rule.condition {
                        let n = ratings[con.category as usize];
                        let condition_passed = if con.greater_than {
                            n > con.target
                        } else {
                            n < con.target
                        };
                        if condition_passed {
                            current_workflow = rule.destination;
                            break;
                        }
                    } else {
                        current_workflow = rule.destination;
                        break;
                    }
                }
            }
            match current_workflow {
                "A" => ratings.into_iter().sum(),
                "R" => 0,
                _ => panic!("final workflow not A or R"),
            }
        })
        .sum()
}

fn count_combos(
    combo: [(u64, u64); 4], 
    name: &str, 
    workflows: &HashMap<&str, Vec<Rule>>
) -> Vec<[(u64, u64); 4]> {
    
    match name {
        "A" => return vec![combo],
        "R" => return vec![],
        _ => {},
    }

    let mut valid_combos = vec![];
    let mut current_combo = combo;
    for rule in workflows.get(name).unwrap() {
        let dest = rule.destination;

        if let Some(condition) = &rule.condition {
            if condition.greater_than {
                if current_combo[condition.category as usize].0 > condition.target {
                    valid_combos.extend(count_combos(current_combo, dest, workflows))
                } else if current_combo[condition.category as usize].1 > condition.target {
                    let mut new = current_combo;
                    new[condition.category as usize].0 = condition.target + 1;
                    valid_combos.extend(count_combos(new, dest, workflows));
                    current_combo[condition.category as usize].1 = condition.target;
                }
            } else {
                if current_combo[condition.category as usize].1 < condition.target {
                    valid_combos.extend(count_combos(current_combo, dest, workflows))
                } else if current_combo[condition.category as usize].0 < condition.target {
                    let mut new = current_combo;
                    new[condition.category as usize].1 = condition.target - 1;
                    valid_combos.extend(count_combos(new, dest, workflows));
                    current_combo[condition.category as usize].0 = condition.target;
                }
            }

        } else {
            valid_combos.extend(count_combos(current_combo, dest, workflows))
        }
    }
    
    valid_combos
}

pub fn part_2(input: &str) -> u64 {
    let workflows = get_workflows(input.split_once("\n\n").unwrap().0);
    let initial_combos = [(1, 4000), (1, 4000), (1, 4000), (1, 4000)];
    let c = count_combos(initial_combos, "in", &workflows);
    dbg!(&c);
    c
        .into_iter()
        .map(|c| {
            c.iter().fold(1, |acc, &(a, b)| acc * (b - a + 1))
        })
        .sum()
}