use itertools::Itertools;
use lazy_static::lazy_static;
use std::iter::Chain;
use std::slice::Iter;
use std::str::FromStr;

use crate::input::{read_lines, FilterNotEmpty, ParseExt};
use crate::Solution;

mod input;

lazy_static! {
    static ref RUCKSACKS: Vec<Rucksack> = read_lines(input::INPUT)
        .filter_not_empty()
        .parse()
        .collect();
}

pub struct Day3;

impl Solution for Day3 {
    fn day(&self) -> u8 {
        3
    }

    fn part_one(&self) -> String {
        format!(
            "Sum of the priorities of item in both compartment of a rucksack: {}",
            sum_priorities_of_item_in_both_compartment(&RUCKSACKS),
        )
    }

    fn part_two(&self) -> String {
        format!("Sum of all group badges: {}", sum_of_all_badges(&RUCKSACKS))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Item(char);

const LOWERCASE_A_PRIORITY: u32 = 1;
const UPPERCASE_A_PRIORITY: u32 = 27;
const LOWERCASE_CHAR_PRIORITY_DIFFERENCE: u32 = 'a' as u32 - LOWERCASE_A_PRIORITY;
const UPPERCASE_CHAR_PRIORITY_DIFFERENCE: u32 = 'A' as u32 - UPPERCASE_A_PRIORITY;

impl Item {
    fn priority(&self) -> u32 {
        let difference = match self.0 {
            'a'..='z' => LOWERCASE_CHAR_PRIORITY_DIFFERENCE,
            'A'..='Z' => UPPERCASE_CHAR_PRIORITY_DIFFERENCE,
            _ => panic!("Invalid item: {}", self.0),
        };
        self.0 as u32 - difference
    }
}

#[derive(Debug)]
struct Rucksack {
    compartment_1: Vec<Item>,
    compartment_2: Vec<Item>,
}

impl Rucksack {
    fn items_in_both_compartments(&self) -> impl Iterator<Item = &Item> + '_ {
        self.compartment_1
            .iter()
            .filter(|item| self.compartment_2.contains(item))
            .unique()
    }

    fn contains(&self, item: &Item) -> bool {
        self.compartment_1.contains(item) || self.compartment_2.contains(item)
    }

    fn iter(&self) -> Chain<Iter<'_, Item>, Iter<'_, Item>> {
        self.compartment_1.iter().chain(self.compartment_2.iter())
    }
}

impl FromStr for Rucksack {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (compartment_1, compartment_2) = line.split_at(line.len() / 2);
        Ok(Rucksack {
            compartment_1: compartment_1.chars().map(Item).collect(),
            compartment_2: compartment_2.chars().map(Item).collect(),
        })
    }
}

fn sum_priorities_of_item_in_both_compartment(rucksacks: &[Rucksack]) -> u32 {
    rucksacks
        .iter()
        .flat_map(|rucksack| rucksack.items_in_both_compartments())
        .map(Item::priority)
        .sum()
}

fn find_badge(rucksacks: &[Rucksack]) -> Option<Item> {
    if let Some((rucksack, tail)) = rucksacks.split_first() {
        rucksack
            .iter()
            .find(|item| {
                tail.iter()
                    .all(|other_rucksack| other_rucksack.contains(item))
            })
            .copied()
    } else {
        None
    }
}

fn find_all_badges(rucksacks: &[Rucksack]) -> impl Iterator<Item = Item> + '_ {
    rucksacks.chunks_exact(3).flat_map(find_badge)
}

fn sum_of_all_badges(rucksacks: &[Rucksack]) -> u32 {
    find_all_badges(rucksacks)
        .map(|badge| badge.priority())
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::input::read_lines;

    const EXAMPLE: &str = r"
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";
    lazy_static! {
        static ref EXAMPLE_RUCKSACKS: Vec<Rucksack> = read_lines(EXAMPLE.as_bytes())
            .filter_not_empty()
            .parse()
            .collect::<Vec<_>>();
    }

    #[test]
    fn part1_example() {
        let result = sum_priorities_of_item_in_both_compartment(&EXAMPLE_RUCKSACKS);

        assert_eq!(result, 157);
    }

    #[test]
    fn part2_example() {
        let result = sum_of_all_badges(&EXAMPLE_RUCKSACKS);

        assert_eq!(result, 70);
    }

    #[test]
    fn example_first_group_badge() {
        let first_group = &EXAMPLE_RUCKSACKS[0..3];

        let result = find_badge(first_group);

        assert_eq!(result, Some(Item('r')));
    }
}
