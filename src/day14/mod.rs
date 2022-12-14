use itertools::Itertools;
use lazy_static::lazy_static;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::RangeInclusive;
use std::str::FromStr;

use crate::input::{read_lines, FilterNotEmpty, ParseExt};
use crate::Solution;

mod input;

lazy_static! {
    static ref ROCKS: Vec<Rock> = read_lines(input::INPUT)
        .filter_not_empty()
        .parse()
        .collect();
}

pub struct Day14;

impl Solution for Day14 {
    fn day(&self) -> u8 {
        14
    }

    fn part_one(&self) -> String {
        let cave = AbyssCave::new(ROCKS.clone());
        format!(
            "Number of resting sand units in cave with abyss: {}",
            cave.last().unwrap(),
        )
    }

    fn part_two(&self) -> String {
        let cave = FloorCave::new(ROCKS.clone());
        format!(
            "Number of resting sand units in cave with floor: {}",
            cave.last().unwrap(),
        )
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Coordinate {
    x: u32,
    y: u32,
}

impl Coordinate {
    const fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    fn lower_coordinates(&self) -> [Coordinate; 3] {
        [
            Coordinate::new(self.x, self.y + 1),
            Coordinate::new(self.x - 1, self.y + 1),
            Coordinate::new(self.x + 1, self.y + 1),
        ]
    }
}

#[derive(Debug, Clone)]
enum Line {
    Horizontal { x: RangeInclusive<u32>, y: u32 },
    Vertical { x: u32, y: RangeInclusive<u32> },
}

impl Line {
    fn new(c1: &Coordinate, c2: &Coordinate) -> Self {
        if c1.x == c2.x {
            Self::Vertical {
                x: c1.x,
                y: range(c1.y, c2.y),
            }
        } else {
            Self::Horizontal {
                x: range(c1.x, c2.x),
                y: c1.y,
            }
        }
    }

    fn all_coordinates(&self) -> Vec<Coordinate> {
        match self {
            Self::Horizontal { x, y } => x.clone().map(|x| Coordinate::new(x, *y)).collect(),
            Self::Vertical { x, y } => y.clone().map(|y| Coordinate::new(*x, y)).collect(),
        }
    }
}

fn range(a: u32, b: u32) -> RangeInclusive<u32> {
    if a < b {
        a..=b
    } else {
        b..=a
    }
}

#[derive(Debug, Clone)]
struct Rock(Vec<Line>);

impl FromStr for Rock {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if let Ok((_, coordinates)) = separated_list1(tag(" -> "), coordinate)(line) {
            let lines = coordinates
                .iter()
                .tuple_windows()
                .map(|(c1, c2)| Line::new(c1, c2))
                .collect();
            Ok(Rock(lines))
        } else {
            Err(format!("Invalid rock : {line}"))
        }
    }
}

fn coordinate(input: &str) -> IResult<&str, Coordinate> {
    let (input, (x, _, y)) = tuple((digit1, tag(","), digit1))(input)?;
    Ok((
        input,
        Coordinate::new(x.parse().unwrap(), y.parse().unwrap()),
    ))
}

#[derive(Debug, Clone)]
struct AbyssCave {
    rocks: HashSet<Coordinate>,
    sands: HashSet<Coordinate>,
    abyss: u32,
}

impl AbyssCave {
    fn new(rocks: Vec<Rock>) -> Self {
        let rocks = rocks
            .into_iter()
            .flat_map(|rock| rock.0)
            .flat_map(|line| line.all_coordinates())
            .collect::<HashSet<_>>();
        let abyss = rocks.iter().map(|c| c.y).max().unwrap();
        Self {
            rocks,
            sands: HashSet::new(),
            abyss,
        }
    }

    fn is_occupied(&self, coordinate: &Coordinate) -> bool {
        self.sands.contains(coordinate) || self.rocks.contains(coordinate)
    }
}

static SAND_ENTRY_POINT: Coordinate = Coordinate::new(500, 0);

impl Iterator for AbyssCave {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let mut sand_unit = SAND_ENTRY_POINT;
        while let Some(coordinate) = sand_unit
            .lower_coordinates()
            .into_iter()
            .find(|c| !self.is_occupied(c))
        {
            if coordinate.y >= self.abyss {
                return None;
            }
            sand_unit = coordinate;
        }
        self.sands.insert(sand_unit);
        Some(self.sands.len())
    }
}

#[derive(Debug, Clone)]
struct FloorCave {
    rocks: HashSet<Coordinate>,
    sands: HashSet<Coordinate>,
    floor: u32,
}

impl FloorCave {
    fn new(rocks: Vec<Rock>) -> Self {
        let rocks = rocks
            .into_iter()
            .flat_map(|rock| rock.0)
            .flat_map(|line| line.all_coordinates())
            .collect::<HashSet<_>>();
        let floor = rocks.iter().map(|c| c.y).max().unwrap() + 2;
        Self {
            rocks,
            sands: HashSet::new(),
            floor,
        }
    }

    fn is_occupied(&self, coordinate: &Coordinate) -> bool {
        coordinate.y >= self.floor
            || self.sands.contains(coordinate)
            || self.rocks.contains(coordinate)
    }
}

impl Iterator for FloorCave {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_occupied(&SAND_ENTRY_POINT) {
            None
        } else {
            let mut sand_unit = SAND_ENTRY_POINT;
            while let Some(coordinate) = sand_unit
                .lower_coordinates()
                .into_iter()
                .find(|c| !self.is_occupied(c))
            {
                sand_unit = coordinate;
            }
            self.sands.insert(sand_unit);
            Some(self.sands.len())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &[u8] = b"
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

    lazy_static! {
        static ref EXAMPLE_ROCKS: Vec<Rock> =
            read_lines(EXAMPLE).filter_not_empty().parse().collect();
    }

    #[test]
    fn part1_example() {
        let cave = AbyssCave::new(EXAMPLE_ROCKS.clone());
        assert_eq!(cave.last().unwrap(), 24);
    }

    #[test]
    fn part2_example() {
        let cave = FloorCave::new(EXAMPLE_ROCKS.clone());
        assert_eq!(cave.last().unwrap(), 93);
    }
}
