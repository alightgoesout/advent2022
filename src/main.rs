extern crate core;

use std::collections::HashMap;
use std::env;
use std::time::Instant;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod input;

trait Solution {
    fn day(&self) -> u8;
    fn part_one(&self) -> String;
    fn part_two(&self) -> String;

    fn execute(&self) {
        let day = self.day();
        let start = Instant::now();
        println!("{day}:1 — {}", self.part_one());
        let part1_duration = start.elapsed();
        println!("Part 1 in {}ms", part1_duration.as_millis());
        println!("{day}:2 — {}", self.part_two());
        let part2_duration = start.elapsed() - part1_duration;
        println!("Part 1 in {}ms", part2_duration.as_millis());
        let total_duration = start.elapsed();
        println!("Done in {}ms", total_duration.as_millis());
    }
}

fn read_day_from_args() -> Option<u8> {
    env::args().nth(1).and_then(|arg| arg.parse().ok())
}

fn solutions() -> HashMap<u8, Box<dyn Solution>> {
    [
        Box::new(day1::Day1) as Box<dyn Solution>,
        Box::new(day2::Day2),
        Box::new(day3::Day3),
        Box::new(day4::Day4),
        Box::new(day5::Day5),
        Box::new(day6::Day6),
        Box::new(day7::Day7),
        Box::new(day8::Day8),
        Box::new(day9::Day9),
        Box::new(day10::Day10),
        Box::new(day11::Day11),
        Box::new(day12::Day12),
        Box::new(day13::Day13),
        Box::new(day14::Day14),
    ]
    .into_iter()
    .map(|solution| (solution.day(), solution))
    .collect()
}

fn main() {
    let solutions = solutions();
    if let Some(solution) = read_day_from_args().and_then(|day| solutions.get(&day)) {
        solution.execute()
    }
}
