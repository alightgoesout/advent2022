use std::ops::RangeInclusive;
use std::str::FromStr;

use lazy_static::lazy_static;

use crate::input::{read_lines, FilterNotEmpty, ParseExt};
use crate::Solution;

mod input;

lazy_static! {
    static ref ASSIGNMENT_PAIRS: Vec<AssignmentPair> = read_lines(input::INPUT)
        .filter_not_empty()
        .parse()
        .collect();
}

pub struct Day4;

impl Solution for Day4 {
    fn day(&self) -> u8 {
        4
    }

    fn part_one(&self) -> String {
        format!(
            "Number of pairs with complete overlap: {}",
            compute_pairs_with_complete_overlap(&ASSIGNMENT_PAIRS),
        )
    }

    fn part_two(&self) -> String {
        format!(
            "Number of pairs with overlap: {}",
            compute_pairs_with_overlap(&ASSIGNMENT_PAIRS),
        )
    }
}

struct Assignment(RangeInclusive<u32>);

impl Assignment {
    fn contains(&self, Assignment(assignment): &Assignment) -> bool {
        self.0.contains(assignment.start()) && self.0.contains(assignment.end())
    }

    fn overlaps(&self, Assignment(assignment): &Assignment) -> bool {
        self.0.contains(assignment.start())
            || self.0.contains(assignment.end())
            || assignment.contains(self.0.start())
    }
}

impl FromStr for Assignment {
    type Err = String;

    fn from_str(assignment: &str) -> Result<Self, Self::Err> {
        if let Some((start, end)) = assignment.split_once('-') {
            let start = start
                .parse()
                .map_err(|_| format!("Invalid start: {start}"))?;
            let end = end.parse().map_err(|_| format!("Invalid end: {end}"))?;
            Ok(Assignment(start..=end))
        } else {
            Err(format!("Invalid assignment: {assignment}"))
        }
    }
}

struct AssignmentPair(Assignment, Assignment);

impl FromStr for AssignmentPair {
    type Err = String;

    fn from_str(pair: &str) -> Result<Self, Self::Err> {
        if let Some((first, second)) = pair.split_once(',') {
            Ok(AssignmentPair(first.parse()?, second.parse()?))
        } else {
            Err(format!("Invalid assignment pair: {pair}"))
        }
    }
}

fn compute_pairs_with_complete_overlap(pairs: &[AssignmentPair]) -> usize {
    pairs
        .iter()
        .filter(|AssignmentPair(first, second)| first.contains(second) || second.contains(first))
        .count()
}

fn compute_pairs_with_overlap(pairs: &[AssignmentPair]) -> usize {
    pairs
        .iter()
        .filter(|AssignmentPair(first, second)| first.overlaps(second))
        .count()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::input::read_lines;
    use lazy_static::lazy_static;

    const EXAMPLE: &str = r"
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    lazy_static! {
        static ref EXAMPLE_PAIRS: Vec<AssignmentPair> = read_lines(EXAMPLE.as_bytes())
            .filter_not_empty()
            .parse()
            .collect();
    }

    #[test]
    fn part1_example() {
        let result = compute_pairs_with_complete_overlap(&EXAMPLE_PAIRS);

        assert_eq!(result, 2);
    }

    #[test]
    fn part2_example() {
        let result = compute_pairs_with_overlap(&EXAMPLE_PAIRS);

        assert_eq!(result, 4);
    }
}
