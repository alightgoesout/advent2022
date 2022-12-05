use crate::input::read_lines;
use crate::Solution;
use lazy_static::lazy_static;

mod input;

lazy_static! {
    static ref CALORIES: Vec<Calories> = parse_calories(read_lines(input::INPUT));
}

pub struct Day1;

impl Solution for Day1 {
    fn day(&self) -> u8 {
        1
    }

    fn part_one(&self) -> String {
        format!(
            "Maximum calories held by one Elf: {}",
            compute_max_calories(&CALORIES),
        )
    }

    fn part_two(&self) -> String {
        format!(
            "Sum of top three calories held by Elves: {}",
            compute_top_three_calories(&CALORIES),
        )
    }
}

type Calories = Vec<u32>;

fn parse_calories(lines: impl Iterator<Item = String>) -> Vec<Calories> {
    let mut calories = Vec::<Calories>::new();
    let mut current = Calories::new();

    for line in lines {
        if line.is_empty() {
            if !current.is_empty() {
                calories.push(current);
                current = Calories::new();
            }
        } else {
            current.push(line.parse().unwrap());
        }
    }

    if !current.is_empty() {
        calories.push(current);
    }

    calories
}

fn compute_max_calories(all_calories: &[Calories]) -> u32 {
    all_calories
        .iter()
        .map(|calories| calories.iter().sum::<u32>())
        .max()
        .unwrap()
}

fn compute_top_three_calories(all_calories: &[Calories]) -> u32 {
    let mut calories = all_calories
        .iter()
        .map(|calories| calories.iter().sum::<u32>())
        .collect::<Vec<_>>();
    calories.sort();
    calories.reverse();
    calories.iter().take(3).sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::input::read_lines;

    static SAMPLE: &str = r"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

    #[test]
    fn part1_example() {
        let all_calories = parse_calories(read_lines(SAMPLE.as_bytes()));

        let result = compute_max_calories(&all_calories);

        assert_eq!(result, 24000);
    }

    #[test]
    fn part2_example() {
        let all_calories = parse_calories(read_lines(SAMPLE.as_bytes()));

        let result = compute_top_three_calories(&all_calories);

        assert_eq!(result, 45000);
    }
}
