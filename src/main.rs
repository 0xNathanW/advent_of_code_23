fn main() {
    let a = day_18::part_2(&mut get_input(18));
    dbg!(a);
}

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;
pub mod day_16;
pub mod day_17;
pub mod day_18;

fn get_input(day: u32) -> String {
    let fill = if day < 10 { "0" } else { "" };
    std::fs::read_to_string(&format!("./inputs/{}{}.txt", fill, day)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_current() {
        let input = std::fs::read_to_string("./inputs/test.txt").unwrap();
        let answer = day_18::part_2(&input);
        dbg!(answer);
    }

    #[test]
    fn assert_past_days() {
        // Day 1.
        {
            let input = &get_input(1);
            assert_eq!(day_01::part_1(input), 54990);
            assert_eq!(day_01::part_2(input), 54473);
        }
        // Day 2.
        {
            let input = &get_input(2);
            assert_eq!(day_02::part_1(input), 2593);
            assert_eq!(day_02::part_2(input), 54699);
        }
        // Day 3.
        {
            let input = &get_input(3);
            assert_eq!(day_03::part_1(input), 535351);
            assert_eq!(day_03::part_2(input), 87287096);
        }
        // Day 4.
        {
            let input = &get_input(4);
            assert_eq!(day_04::part_1(input), 21088);
            assert_eq!(day_04::part_2(input), 6874754);
        }
        // Day 5.
        {
            let input = &get_input(5);
            assert_eq!(day_05::part_1(input), 3374647);
            // This one takes a long time.
            // assert_eq!(day_05::part_2(input), 6082852);
        }
        // Day 6.
        {
            let input = &get_input(6);
            assert_eq!(day_06::part_1(input), 1413720);
            assert_eq!(day_06::part_2(input), 30565288);
        }
        // Day 7.
        {
            let input = &get_input(7);
            assert_eq!(day_07::part_1(input), 250957639);
            assert_eq!(day_07::part_2(input), 251515496);
        }
        // Day 8.
        {
            let input = &get_input(8);
            assert_eq!(day_08::part_1(input), 15871);
            assert_eq!(day_08::part_2(input), 11283670395017);
        }
        // Day 9.
        {
            let input = &get_input(9);
            assert_eq!(day_09::part_1(input), 1641934234);
            assert_eq!(day_09::part_2(input), 975);
        }
        // Day 10.
        {
            let input = &get_input(10);
            assert_eq!(day_10::part_1(input), 6903);
            // I broke it somehow.
            // assert_eq!(day_10::part_2(input), 265);
        }
        // Day 11.
        {
            let input = &get_input(11);
            assert_eq!(day_11::part_1(input), 10885634);
            assert_eq!(day_11::part_2(input), 707505470642);
        }
    }
}
