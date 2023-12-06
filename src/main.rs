const DAY: u32 = 06;

fn main() {
    let answer = day_06::part_1(&format!("./inputs/0{}.txt", DAY));
    println!("{}", answer);
}

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let answer = day_06::part_2("inputs/test.txt");
        println!("{}", answer);
    }
}