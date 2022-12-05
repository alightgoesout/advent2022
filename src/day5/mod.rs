use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::input::{read_lines, FilterNotEmpty, ParseExt};
use crate::Solution;

mod input;

lazy_static! {
    static ref STACKS: Stacks = parse_stacks(read_lines(input::STACKS).filter_not_empty());
    static ref INSTRUCTIONS: Vec<MoveInstruction> = read_lines(input::INSTRUCTIONS)
        .filter_not_empty()
        .parse()
        .collect();
}

pub struct Day5;

impl Solution for Day5 {
    fn day(&self) -> u8 {
        5
    }

    fn part_one(&self) -> String {
        let mut stacks = STACKS.clone();
        stacks.move_all_with_crate_mover_9000(&INSTRUCTIONS);
        format!(
            "Top crates after all moves with CrateMover 9000: {}",
            crates_to_string(&compute_top_crates(&stacks)),
        )
    }

    fn part_two(&self) -> String {
        let mut stacks = STACKS.clone();
        stacks.move_all_with_crate_mover_9001(&INSTRUCTIONS);
        format!(
            "Top crates after all moves with CrateMover 9001: {}",
            crates_to_string(&compute_top_crates(&stacks)),
        )
    }
}

fn compute_top_crates(stacks: &Stacks) -> Vec<Option<Crate>> {
    stacks
        .0
        .iter()
        .map(|column| column.last().copied())
        .collect()
}

fn crates_to_string(crates: &[Option<Crate>]) -> String {
    crates
        .iter()
        .map(|o| o.map(|c| c.0).unwrap_or(' '))
        .join("")
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Crate(char);

impl Display for Crate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Stacks(Vec<Vec<Crate>>);

impl Display for Stacks {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for stack in &self.0 {
            for crate_ in stack {
                crate_.fmt(f)?;
                write!(f, " ")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Stacks {
    pub fn move_with_crate_mover_9000(
        &mut self,
        &MoveInstruction { number, from, to }: &MoveInstruction,
    ) {
        for _ in 0..number {
            self.move_once(from, to)
        }
    }

    fn move_once(&mut self, from: usize, to: usize) {
        if let Some(crate_) = self.0.get_mut(from).and_then(Vec::pop) {
            if let Some(column) = self.0.get_mut(to) {
                column.push(crate_);
            }
        }
    }

    pub fn move_all_with_crate_mover_9000(&mut self, instructions: &[MoveInstruction]) {
        for instruction in instructions {
            self.move_with_crate_mover_9000(instruction);
        }
    }

    pub fn move_with_crate_mover_9001(
        &mut self,
        &MoveInstruction { number, from, to }: &MoveInstruction,
    ) {
        if let Some(mut moved_crates) = self
            .0
            .get_mut(from)
            .map(|origin_stack| origin_stack.split_off(origin_stack.len().saturating_sub(number)))
        {
            if let Some(target_stack) = self.0.get_mut(to) {
                target_stack.append(&mut moved_crates)
            }
        }
    }

    pub fn move_all_with_crate_mover_9001(&mut self, instructions: &[MoveInstruction]) {
        for instruction in instructions {
            self.move_with_crate_mover_9001(instruction);
        }
    }
}

fn parse_stack(line: &str) -> Vec<Crate> {
    line.split(' ')
        .flat_map(|c| c.chars().nth(1))
        .map(Crate)
        .collect()
}

fn parse_stacks(lines: impl Iterator<Item = String>) -> Stacks {
    Stacks(lines.map(|line| parse_stack(&line)).collect())
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct MoveInstruction {
    number: usize,
    from: usize,
    to: usize,
}

lazy_static! {
    static ref MOVE_INSTRUCTION_REGEX: Regex =
        Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
}

impl FromStr for MoveInstruction {
    type Err = String;

    fn from_str(instruction: &str) -> Result<Self, Self::Err> {
        if let Some(captures) = MOVE_INSTRUCTION_REGEX.captures(instruction) {
            Ok(MoveInstruction {
                number: captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                from: captures.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1,
                to: captures.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1,
            })
        } else {
            Err(format!("Invalid instruction: {instruction}"))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_STACKS_INPUT: &str = r"
[Z] [N]
[M] [C] [D]
[P]
";

    const EXAMPLE_INSTRUCTIONS_INPUT: &str = r"
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    lazy_static! {
        static ref EXAMPLE_STACKS: Stacks =
            parse_stacks(read_lines(EXAMPLE_STACKS_INPUT.as_bytes()).filter_not_empty());
        static ref EXAMPLE_INSTRUCTIONS: Vec<MoveInstruction> =
            read_lines(EXAMPLE_INSTRUCTIONS_INPUT.as_bytes())
                .filter_not_empty()
                .parse()
                .collect();
    }

    #[test]
    fn parse_example_stacks() {
        assert_eq!(
            *EXAMPLE_STACKS,
            Stacks(vec![
                vec![Crate('Z'), Crate('N')],
                vec![Crate('M'), Crate('C'), Crate('D')],
                vec![Crate('P')]
            ]),
        );
    }

    #[test]
    fn parse_input_stacks() {
        assert_eq!(
            STACKS.to_string(),
            r"[V] [C] [D] [R] [Z] [G] [B] [W] 
[G] [W] [F] [C] [B] [S] [T] [V] 
[C] [B] [S] [N] [W] 
[Q] [G] [M] [N] [J] [V] [C] [P] 
[T] [S] [L] [F] [D] [H] [B] 
[J] [V] [T] [W] [M] [N] 
[P] [F] [L] [C] [S] [T] [G] 
[B] [D] [Z] 
[M] [N] [Z] [W] 
",
        );
    }

    #[test]
    fn parse_example_instructions() {
        assert_eq!(
            *EXAMPLE_INSTRUCTIONS,
            vec![
                MoveInstruction {
                    number: 1,
                    from: 1,
                    to: 0
                },
                MoveInstruction {
                    number: 3,
                    from: 0,
                    to: 2
                },
                MoveInstruction {
                    number: 2,
                    from: 1,
                    to: 0
                },
                MoveInstruction {
                    number: 1,
                    from: 0,
                    to: 1
                },
            ],
        )
    }

    #[test]
    fn test_move_with_crate_mover_9000() {
        let mut stacks = EXAMPLE_STACKS.clone();

        stacks.move_with_crate_mover_9000(&MoveInstruction {
            number: 2,
            from: 1,
            to: 0,
        });

        assert_eq!(
            stacks,
            Stacks(vec![
                vec![Crate('Z'), Crate('N'), Crate('D'), Crate('C')],
                vec![Crate('M')],
                vec![Crate('P')]
            ]),
        )
    }

    #[test]
    fn test_move_with_crate_mover_9001() {
        let mut stacks = EXAMPLE_STACKS.clone();

        stacks.move_with_crate_mover_9001(&MoveInstruction {
            number: 2,
            from: 1,
            to: 0,
        });

        assert_eq!(
            stacks,
            Stacks(vec![
                vec![Crate('Z'), Crate('N'), Crate('C'), Crate('D')],
                vec![Crate('M')],
                vec![Crate('P')]
            ]),
        )
    }

    #[test]
    fn part1_example() {
        let mut stacks = EXAMPLE_STACKS.clone();

        stacks.move_all_with_crate_mover_9000(&EXAMPLE_INSTRUCTIONS);
        let result = compute_top_crates(&stacks);

        assert_eq!(
            result,
            vec![Some(Crate('C')), Some(Crate('M')), Some(Crate('Z'))],
        );
    }
}
