use std::collections::{HashMap, HashSet};
use std::time::Duration;

use lazy_static::lazy_static;
use termion::{clear, color};

use crate::input::{read_lines, FilterNotEmpty};
use crate::Solution;

mod input;

lazy_static! {
    static ref HEIGHT_MAP: HeightMap<41, 132> =
        HeightMap::parse(read_lines(input::INPUT).filter_not_empty());
}

pub struct Day12;

impl Solution for Day12 {
    fn day(&self) -> u8 {
        12
    }

    fn part_one(&self) -> String {
        format!(
            "Shortest path: {}",
            HEIGHT_MAP
                .shortest_path(HEIGHT_MAP.start, true, |p| p == HEIGHT_MAP.end, false)
                .unwrap(),
        )
    }

    fn part_two(&self) -> String {
        format!(
            "Shortest a to end: {}",
            HEIGHT_MAP
                .shortest_path(
                    HEIGHT_MAP.end,
                    false,
                    |p| HEIGHT_MAP.height(&p) == b'a',
                    false
                )
                .unwrap(),
        )
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Default, Hash)]
struct Position {
    row: usize,
    column: usize,
}

impl Position {
    fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct HeightMap<const ROWS: usize, const COLUMNS: usize> {
    start: Position,
    end: Position,
    heights: [[char; COLUMNS]; ROWS],
}

impl<const ROWS: usize, const COLUMNS: usize> HeightMap<ROWS, COLUMNS> {
    pub fn parse(lines: impl Iterator<Item = String>) -> Self {
        let mut start = Position::default();
        let mut end = Position::default();
        let mut heights = [['a'; COLUMNS]; ROWS];

        for (row, line) in lines.take(ROWS).enumerate() {
            for (column, char) in line.chars().take(COLUMNS).enumerate() {
                let height = match char {
                    'S' => {
                        start = Position::new(row, column);
                        'a'
                    }
                    'E' => {
                        end = Position::new(row, column);
                        'z'
                    }
                    _ => char,
                };
                heights[row][column] = height;
            }
        }

        Self {
            start,
            end,
            heights,
        }
    }

    fn shortest_path<E>(
        &self,
        start: Position,
        forward: bool,
        end_condition: E,
        visualization: bool,
    ) -> Option<usize>
    where
        E: Fn(Position) -> bool,
    {
        let mut visited = HashSet::new();
        let mut shortest_paths: HashMap<Position, usize> = [(start, 0)].into();

        while let Some((&position, &shortest_path)) = shortest_paths
            .iter()
            .filter(|(position, _)| !visited.contains(*position))
            .min_by_key(|(_, path)| **path)
        {
            for neighbor in self.get_neighbors(&position, forward) {
                if end_condition(neighbor) {
                    return Some(shortest_path + 1);
                }
                shortest_paths
                    .entry(neighbor)
                    .and_modify(|current| *current = (*current).min(shortest_path + 1))
                    .or_insert(shortest_path + 1);
            }
            visited.insert(position);
            if visualization {
                self.print(&visited, &shortest_paths);
                std::thread::sleep(Duration::from_millis(50))
            }
        }

        shortest_paths.get(&self.end).copied()
    }

    fn print(&self, visited: &HashSet<Position>, shortest_paths: &HashMap<Position, usize>) {
        println!("{}", clear::All);
        for row in 0..ROWS {
            for column in 0..COLUMNS {
                let position = Position { row, column };

                if position == self.start {
                    print!("{}", color::Fg(color::Magenta));
                } else if position == self.end {
                    print!("{}", color::Fg(color::Yellow));
                } else if visited.contains(&position) {
                    print!("{}", color::Fg(color::Green));
                } else {
                    print!("{}", color::Fg(color::Red));
                }
                if let Some(shortest_path) = shortest_paths.get(&position) {
                    print!("{:3}", shortest_path);
                } else {
                    print!("  ?");
                }
            }
            println!()
        }
    }

    fn get_neighbors(&self, position: &Position, forward: bool) -> Vec<Position> {
        let mut neighbors = Vec::new();

        if position.row > 0 {
            let new_position = Position::new(position.row - 1, position.column);
            if self.can_move(position, &new_position, forward) {
                neighbors.push(new_position)
            }
        }
        if position.row < ROWS - 1 {
            let new_position = Position::new(position.row + 1, position.column);
            if self.can_move(position, &new_position, forward) {
                neighbors.push(new_position)
            }
        }
        if position.column > 0 {
            let new_position = Position::new(position.row, position.column - 1);
            if self.can_move(position, &new_position, forward) {
                neighbors.push(new_position)
            }
        }
        if position.column < COLUMNS - 1 {
            let new_position = Position::new(position.row, position.column + 1);
            if self.can_move(position, &new_position, forward) {
                neighbors.push(new_position)
            }
        }

        neighbors
    }

    fn can_move(&self, from: &Position, to: &Position, forward: bool) -> bool {
        let from_height = self.height(from);
        let to_height = self.height(to);

        if forward {
            from_height >= to_height || from_height == to_height - 1
        } else {
            from_height <= to_height || from_height - 1 == to_height
        }
    }

    pub fn height(&self, &Position { row, column }: &Position) -> u8 {
        self.heights[row][column] as u8
    }
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &[u8] = b"
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[test]
    fn parse_example() {
        let height_map = HeightMap::<5, 8>::parse(read_lines(EXAMPLE).filter_not_empty());

        assert_eq!(
            height_map,
            HeightMap {
                start: Position { row: 0, column: 0 },
                end: Position { row: 2, column: 5 },
                heights: [
                    ['a', 'a', 'b', 'q', 'p', 'o', 'n', 'm'],
                    ['a', 'b', 'c', 'r', 'y', 'x', 'x', 'l'],
                    ['a', 'c', 'c', 's', 'z', 'z', 'x', 'k'],
                    ['a', 'c', 'c', 't', 'u', 'v', 'w', 'j'],
                    ['a', 'b', 'd', 'e', 'f', 'g', 'h', 'i'],
                ],
            }
        )
    }

    #[test]
    fn part1_example() {
        let height_map = HeightMap::<5, 8>::parse(read_lines(EXAMPLE).filter_not_empty());

        let result =
            height_map.shortest_path(height_map.start, true, |p| p == height_map.end, false);

        assert_eq!(result, Some(31));
    }
}
