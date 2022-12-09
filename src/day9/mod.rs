use lazy_static::lazy_static;
use std::collections::HashSet;
use std::str::FromStr;

use crate::day9::Direction::{Down, Left, Right, Up};
use crate::input::{read_lines, FilterNotEmpty, ParseExt};
use crate::Solution;

mod input;

lazy_static! {
    static ref INSTRUCTIONS: Vec<Instruction> = read_lines(input::INPUT)
        .filter_not_empty()
        .parse()
        .collect();
}

pub struct Day9;

impl Solution for Day9 {
    fn day(&self) -> u8 {
        9
    }

    fn part_one(&self) -> String {
        let mut rope = Rope::<2>::default();
        let tail_positions = rope.execute_all(&INSTRUCTIONS);
        format!(
            "Number of different positions of the two knots rope tail: {}",
            tail_positions.len(),
        )
    }

    fn part_two(&self) -> String {
        let mut rope = Rope::<10>::default();
        let tail_positions = rope.execute_all(&INSTRUCTIONS);
        format!(
            "Number of different positions of the 10 knots rope tail: {}",
            tail_positions.len(),
        )
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default, Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    pub fn is_adjacent(&self, other: &Position) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Rope<const SIZE: usize>([Position; SIZE]);

impl<const SIZE: usize> Default for Rope<SIZE> {
    fn default() -> Self {
        Rope([Position::default(); SIZE])
    }
}

impl<const SIZE: usize> Rope<SIZE> {
    pub fn execute(&mut self, Instruction { direction, steps }: Instruction) -> HashSet<Position> {
        let mut tail_positions = HashSet::new();

        for _ in 0..steps {
            self.move_head(direction);
            tail_positions.insert(self.0[SIZE - 1]);
        }

        tail_positions
    }

    fn move_head(&mut self, direction: Direction) {
        match direction {
            Up => self.0[0].y += 1,
            Down => self.0[0].y -= 1,
            Right => self.0[0].x += 1,
            Left => self.0[0].x -= 1,
        }
        self.move_knots()
    }

    pub fn execute_all(&mut self, instructions: &[Instruction]) -> HashSet<Position> {
        instructions
            .iter()
            .flat_map(|instruction| self.execute(*instruction))
            .collect()
    }

    fn move_knots(&mut self) {
        for i in 1..SIZE {
            let previous_knot = self.0[i - 1];
            let current_knot = &mut self.0[i];

            if !current_knot.is_adjacent(&previous_knot) {
                current_knot.x += (previous_knot.x - current_knot.x).signum();
                current_knot.y += (previous_knot.y - current_knot.y).signum();
            }
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default)]
struct OldRope {
    head: Position,
    tail: Position,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Instruction {
    direction: Direction,
    steps: usize,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(instruction: &str) -> Result<Self, Self::Err> {
        if let Some((direction, steps)) = instruction.split_once(' ') {
            let direction = match direction {
                "U" => Up,
                "D" => Down,
                "R" => Right,
                "L" => Left,
                _ => return Err(format!("Invalid instruction: {instruction}")),
            };
            steps
                .parse()
                .map(|steps| Instruction { direction, steps })
                .map_err(|_| format!("Invalid instruction: {instruction}"))
        } else {
            Err(format!("Invalid instruction: {instruction}"))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const SMALL_EXAMPLE: &[u8] = b"
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    const LARGE_EXAMPLE: &[u8] = b"
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";

    #[test]
    fn part2_small_example() {
        let instructions = read_lines(SMALL_EXAMPLE)
            .filter_not_empty()
            .parse()
            .collect::<Vec<Instruction>>();
        let mut rope = Rope::<10>::default();

        let result = rope.execute_all(&instructions).len();

        assert_eq!(result, 1);
    }

    #[test]
    fn part2_large_example() {
        let instructions = read_lines(LARGE_EXAMPLE)
            .filter_not_empty()
            .parse()
            .collect::<Vec<Instruction>>();
        let mut rope = Rope::<10>::default();

        let result = rope.execute_all(&instructions).len();

        assert_eq!(result, 36);
    }
}
