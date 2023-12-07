const DAY: u32 = 07;

fn main() {
    let a = day_07::part_2(&format!("./inputs/0{}.txt", DAY));
    dbg!(a);
}

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let answer = day_07::part_2("inputs/test.txt");
        println!("{}", answer);
    }
}