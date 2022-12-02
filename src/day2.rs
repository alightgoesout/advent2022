use lazy_static::lazy_static;
use std::str::FromStr;

use crate::input::{read_lines_from_file, ParseExt};
use crate::Solution;

lazy_static! {
    static ref LINES: Vec<String> = read_lines_from_file("day2")
        .filter(|line| !line.is_empty())
        .collect();
}

pub struct Day2;

impl Solution for Day2 {
    fn day(&self) -> u8 {
        2
    }

    fn part_one(&self) -> String {
        let rounds = LINES.iter().parse();
        format!("My score after playing all rounds: {}", play_game(rounds).1,)
    }

    fn part_two(&self) -> String {
        let rounds = LINES
            .iter()
            .parse::<Strategy>()
            .map(|strategy| strategy.into());
        format!(
            "My score after playing all rounds according to the Elf's strategy: {}",
            play_game(rounds).1,
        )
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn defeats(&self, other: &Shape) -> bool {
        other
            == match self {
                Self::Rock => &Self::Scissors,
                Self::Paper => &Self::Rock,
                Self::Scissors => &Self::Paper,
            }
    }

    fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl FromStr for Shape {
    type Err = String;

    fn from_str(shape: &str) -> Result<Self, Self::Err> {
        match shape {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => Err(format!("Unknown shape: {shape}")),
        }
    }
}

enum Outcome {
    Loss,
    Draw,
    Win,
}

impl FromStr for Outcome {
    type Err = String;

    fn from_str(outcome: &str) -> Result<Self, Self::Err> {
        match outcome {
            "X" => Ok(Self::Loss),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(format!("Unknown outcome: {outcome}")),
        }
    }
}

struct Strategy {
    player1_shape: Shape,
    player2_outcome: Outcome,
}

impl FromStr for Strategy {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if let Some((shape, outcome)) = line.split_once(' ') {
            Ok(Strategy {
                player1_shape: shape.parse()?,
                player2_outcome: outcome.parse()?,
            })
        } else {
            Err(format!("The line does not contain two shapes: {line}"))
        }
    }
}

struct Round {
    player1_shape: Shape,
    player2_shape: Shape,
}

impl Round {
    fn scores(&self) -> (u32, u32) {
        if self.player1_shape.defeats(&self.player2_shape) {
            (6 + self.player1_shape.score(), self.player2_shape.score())
        } else if self.player2_shape.defeats(&self.player1_shape) {
            (self.player1_shape.score(), 6 + self.player2_shape.score())
        } else {
            (
                3 + self.player1_shape.score(),
                3 + self.player2_shape.score(),
            )
        }
    }
}

impl From<Strategy> for Round {
    fn from(strategy: Strategy) -> Self {
        let player2_shape = match strategy.player2_outcome {
            Outcome::Loss => match strategy.player1_shape {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            },
            Outcome::Draw => strategy.player1_shape,
            Outcome::Win => match strategy.player1_shape {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
        };
        Round {
            player1_shape: strategy.player1_shape,
            player2_shape,
        }
    }
}

impl FromStr for Round {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if let Some((first, second)) = line.split_once(' ') {
            Ok(Round {
                player1_shape: first.parse()?,
                player2_shape: second.parse()?,
            })
        } else {
            Err(format!("The line does not contain two shapes: {line}"))
        }
    }
}

fn play_game(rounds: impl IntoIterator<Item = Round>) -> (u32, u32) {
    rounds
        .into_iter()
        .map(|round| round.scores())
        .reduce(|(p1_1, p2_1), (p1_2, p2_2)| (p1_1 + p1_2, p2_1 + p2_2))
        .unwrap_or((0, 0))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::input::{read_lines, ParseExt};

    static EXAMPLE: &str = r"
A Y
B X
C Z
";

    #[test]
    fn part1_example() {
        let rounds = read_lines(EXAMPLE.as_bytes())
            .filter(|line| !line.is_empty())
            .parse();

        let result = play_game(rounds);

        assert_eq!(result, (15, 15));
    }

    #[test]
    fn part2_example() {
        let rounds = read_lines(EXAMPLE.as_bytes())
            .filter(|line| !line.is_empty())
            .parse::<Strategy>()
            .map(|strategy| strategy.into());

        let result = play_game(rounds);

        assert_eq!(result, (15, 12));
    }
}
