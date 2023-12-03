fn main() {
    let answer = day_03::part_2("inputs/03.txt");
    println!("{}", answer);
}

pub mod day_01;
pub mod day_02;
pub mod day_03;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let answer = day_03::part_2("inputs/test.txt");
        println!("{}", answer);
    }
}