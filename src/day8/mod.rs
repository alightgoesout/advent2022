use lazy_static::lazy_static;
use std::collections::HashSet;

use crate::input::{read_lines, FilterNotEmpty};
use crate::Solution;
use Direction::{East, North, South, West};

mod input;

lazy_static! {
    static ref TREES: Trees<99> = Trees::parse(read_lines(input::INPUT).filter_not_empty());
}

pub struct Day8;

impl Solution for Day8 {
    fn day(&self) -> u8 {
        8
    }

    fn part_one(&self) -> String {
        format!("Number of visible trees: {}", TREES.visible_trees().len())
    }

    fn part_two(&self) -> String {
        format!("Highest scenic score: {}", TREES.highest_scenic_score())
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Trees<const WIDTH: usize>([[u8; WIDTH]; WIDTH]);

impl<const WIDTH: usize> Trees<WIDTH> {
    fn parse(rows: impl Iterator<Item = String>) -> Self {
        let mut trees = [[0; WIDTH]; WIDTH];

        for (row, row_chars) in rows.take(WIDTH).enumerate() {
            for (column, char) in row_chars.chars().take(WIDTH).enumerate() {
                trees[row][column] = char.to_digit(10).unwrap() as u8;
            }
        }

        Self(trees)
    }

    fn visible_trees(&self) -> HashSet<Tree> {
        let mut visible_trees = HashSet::new();

        for i in 0..WIDTH {
            visible_trees.extend(TreeLineIterator::north(&self.0, i).visible_trees_on_line());
            visible_trees.extend(TreeLineIterator::east(&self.0, i).visible_trees_on_line());
            visible_trees.extend(TreeLineIterator::south(&self.0, i).visible_trees_on_line());
            visible_trees.extend(TreeLineIterator::west(&self.0, i).visible_trees_on_line());
        }

        visible_trees
    }

    fn highest_scenic_score(&self) -> usize {
        let mut max = 0;

        for row in 0..WIDTH {
            for column in 0..WIDTH {
                max = max.max(self.scenic_score(row, column));
            }
        }

        max
    }

    fn scenic_score(&self, row: usize, column: usize) -> usize {
        TreeLineIterator::from(&self.0, row, column, North)
            .visible_trees_from_tree()
            .count()
            * TreeLineIterator::from(&self.0, row, column, East)
                .visible_trees_from_tree()
                .count()
            * TreeLineIterator::from(&self.0, row, column, South)
                .visible_trees_from_tree()
                .count()
            * TreeLineIterator::from(&self.0, row, column, West)
                .visible_trees_from_tree()
                .count()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Tree {
    column: usize,
    row: usize,
    height: u8,
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
struct TreeLineIterator<'a, const WIDTH: usize> {
    trees: &'a [[u8; WIDTH]; WIDTH],
    row: Option<usize>,
    column: Option<usize>,
    direction: Direction,
}

impl<'a, const WIDTH: usize> TreeLineIterator<'a, WIDTH> {
    fn north(trees: &'a [[u8; WIDTH]; WIDTH], column: usize) -> Self {
        Self {
            trees,
            row: Some(WIDTH - 1),
            column: Some(column),
            direction: North,
        }
    }

    fn east(trees: &'a [[u8; WIDTH]; WIDTH], row: usize) -> Self {
        Self {
            trees,
            row: Some(row),
            column: Some(0),
            direction: East,
        }
    }

    fn south(trees: &'a [[u8; WIDTH]; WIDTH], column: usize) -> Self {
        Self {
            trees,
            row: Some(0),
            column: Some(column),
            direction: South,
        }
    }

    fn west(trees: &'a [[u8; WIDTH]; WIDTH], row: usize) -> Self {
        Self {
            trees,
            row: Some(row),
            column: Some(WIDTH - 1),
            direction: West,
        }
    }

    fn from(
        trees: &'a [[u8; WIDTH]; WIDTH],
        row: usize,
        column: usize,
        direction: Direction,
    ) -> Self {
        Self {
            trees,
            row: Some(row),
            column: Some(column),
            direction,
        }
    }

    fn increment(&mut self) {
        match self.direction {
            East => {
                self.column = self
                    .column
                    .filter(|column| *column < WIDTH - 1)
                    .map(|column| column + 1)
            }
            West => {
                self.column = self
                    .column
                    .filter(|column| *column > 0)
                    .map(|column| column - 1);
            }
            South => self.row = self.row.filter(|row| *row < WIDTH - 1).map(|row| row + 1),
            North => self.row = self.row.filter(|row| *row > 0).map(|row| row - 1),
        }
    }
}

impl<'a, const WIDTH: usize> Iterator for TreeLineIterator<'a, WIDTH> {
    type Item = Tree;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.row, self.column) {
            (Some(row), Some(column)) => {
                let tree = Tree {
                    column,
                    row,
                    height: self.trees[row][column],
                };
                self.increment();
                Some(tree)
            }
            _ => None,
        }
    }
}

trait VisibleTreesOnLine {
    type Output: Iterator<Item = Tree>;

    fn visible_trees_on_line(self) -> Self::Output;
}

impl<I> VisibleTreesOnLine for I
where
    I: Iterator<Item = Tree>,
{
    type Output = VisibleTreesOnLineIterator<I>;

    fn visible_trees_on_line(self) -> Self::Output {
        VisibleTreesOnLineIterator {
            iterator: self,
            highest_tree: None,
        }
    }
}

struct VisibleTreesOnLineIterator<I> {
    iterator: I,
    highest_tree: Option<u8>,
}

impl<I> Iterator for VisibleTreesOnLineIterator<I>
where
    I: Iterator<Item = Tree>,
{
    type Item = Tree;

    fn next(&mut self) -> Option<Self::Item> {
        for tree in &mut self.iterator {
            match self.highest_tree {
                None => {
                    self.highest_tree = Some(tree.height);
                    return Some(tree);
                }
                Some(previous_height) if previous_height < tree.height => {
                    self.highest_tree = Some(tree.height);
                    return Some(tree);
                }
                _ => (),
            }
        }

        None
    }
}

trait VisibleTreesFromTree {
    type Output: Iterator<Item = Tree>;

    fn visible_trees_from_tree(self) -> Self::Output;
}

impl<I> VisibleTreesFromTree for I
where
    I: Iterator<Item = Tree>,
{
    type Output = VisibleTreesFromTreeIterator<I>;

    fn visible_trees_from_tree(mut self) -> Self::Output {
        if let Some(tree) = self.next() {
            VisibleTreesFromTreeIterator {
                iterator: self,
                height: tree.height,
                end: false,
            }
        } else {
            VisibleTreesFromTreeIterator {
                iterator: self,
                height: 0,
                end: true,
            }
        }
    }
}

struct VisibleTreesFromTreeIterator<I> {
    iterator: I,
    height: u8,
    end: bool,
}

impl<I> Iterator for VisibleTreesFromTreeIterator<I>
where
    I: Iterator<Item = Tree>,
{
    type Item = Tree;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.end {
            if let Some(tree) = self.iterator.next() {
                if tree.height >= self.height {
                    self.end = true
                }
                return Some(tree);
            } else {
                self.end = true
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &[u8] = b"
30373
25512
65332
33549
35390
";

    lazy_static! {
        static ref EXAMPLE_TREES: Trees<5> = Trees::parse(read_lines(EXAMPLE).filter_not_empty());
    }

    #[test]
    fn parse_example() {
        assert_eq!(
            *EXAMPLE_TREES,
            Trees([
                [3, 0, 3, 7, 3],
                [2, 5, 5, 1, 2],
                [6, 5, 3, 3, 2],
                [3, 3, 5, 4, 9],
                [3, 5, 3, 9, 0],
            ]),
        );
    }

    #[test]
    fn part1_example() {
        let result = EXAMPLE_TREES.visible_trees();

        assert_eq!(result.len(), 21);
    }

    #[test]
    fn test_visible_trees_from_tree() {
        let result = TreeLineIterator::from(&EXAMPLE_TREES.0, 1, 2, North)
            .visible_trees_from_tree()
            .count();

        assert_eq!(result, 1);
    }

    #[test]
    fn test_scenic_score_example1() {
        let result = EXAMPLE_TREES.scenic_score(1, 2);

        assert_eq!(result, 4);
    }

    #[test]
    fn test_scenic_score_example2() {
        let result = EXAMPLE_TREES.scenic_score(3, 2);

        assert_eq!(result, 8);
    }

    #[test]
    fn part2_example() {
        let result = EXAMPLE_TREES.highest_scenic_score();

        assert_eq!(result, 8);
    }
}
