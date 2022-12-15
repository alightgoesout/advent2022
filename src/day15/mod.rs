use itertools::Itertools;
use lazy_static::lazy_static;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{opt, recognize};
use nom::sequence::tuple;
use nom::IResult;
use std::collections::HashSet;
use std::ops::RangeInclusive;
use std::str::FromStr;

use crate::input::{read_lines, FilterNotEmpty, ParseExt};
use crate::Solution;

mod input;

lazy_static! {
    static ref SENSORS: Vec<Sensor> = read_lines(input::INPUT)
        .filter_not_empty()
        .parse()
        .collect();
}

pub struct Day15;

impl Solution for Day15 {
    fn day(&self) -> u8 {
        15
    }

    fn part_one(&self) -> String {
        format!(
            "Number of coordinates without a beacon on row 2 000 000: {}",
            number_of_coordinates_without_beacon_on_row(&SENSORS, 2_000_000),
        )
    }

    fn part_two(&self) -> String {
        format!(
            "Tuning frequency of distress beacon: {}",
            find_missing_beacon_within_zone(&SENSORS, 0, 4_000_000)
                .map(|Coordinate { x, y }| x * 4_000_000 + y)
                .unwrap_or(0),
        )
    }
}

fn number_of_coordinates_without_beacon_on_row(sensors: &[Sensor], row: i64) -> usize {
    ranges_without_beacon_on_row(sensors, row)
        .iter()
        .map(|range| (range.end() - range.start() + 1) as usize)
        .sum::<usize>()
}

fn ranges_without_beacon_on_row(sensors: &[Sensor], row: i64) -> Vec<RangeInclusive<i64>> {
    sensors
        .iter()
        .flat_map(|sensor| sensor.coordinates_without_beacon_on_row(row))
        .sorted_by_key(|range| *range.start())
        .fold(Vec::new(), |mut ranges, range| {
            match ranges.last_mut() {
                Some(last) if RangeInclusive::contains(last, range.start()) => {
                    *last = (*last.start())..=(*range.end().max(last.end()));
                }
                _ => ranges.push(range),
            }
            ranges
        })
}

fn find_missing_beacon_within_zone(sensors: &[Sensor], min: i64, max: i64) -> Option<Coordinate> {
    for row in min..=max {
        let ranges = ranges_without_beacon_on_row(sensors, row);
        let mut possible_beacons = HashSet::new();
        let mut i = min;
        for range in ranges {
            possible_beacons.extend(i..*range.start());
            i = range.end() + 1;
        }
        possible_beacons.extend(i..=max);
        sensors
            .iter()
            .map(|sensor| sensor.beacon)
            .filter(|beacon| beacon.y == row)
            .for_each(|beacon| {
                possible_beacons.remove(&beacon.x);
            });
        if let Some(x) = possible_beacons.iter().next() {
            return Some(Coordinate::new(*x, row));
        }
    }
    None
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
struct Coordinate {
    x: i64,
    y: i64,
}

impl Coordinate {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn distance(&self, &Coordinate { x, y }: &Coordinate) -> i64 {
        (self.x - x).abs() + (self.y - y).abs()
    }
}

struct Sensor {
    position: Coordinate,
    beacon: Coordinate,
}

impl Sensor {
    fn beacon_distance(&self) -> i64 {
        self.position.distance(&self.beacon)
    }

    fn distance_with_row(&self, y: i64) -> i64 {
        (self.position.y - y).abs()
    }

    fn coordinates_without_beacon_on_row(&self, row: i64) -> Option<RangeInclusive<i64>> {
        let distance_with_row = self.distance_with_row(row);
        let beacon_distance = self.beacon_distance();
        if distance_with_row < beacon_distance {
            let n = beacon_distance - distance_with_row;
            let range_start = self.position.x - n;
            let range_end = self.position.x + n;
            if self.beacon.y == row {
                if self.beacon.x == range_start {
                    Some(range_start + 1..=range_end)
                } else {
                    Some(range_start..=range_end - 1)
                }
            } else {
                Some(range_start..=range_end)
            }
        } else {
            None
        }
    }
}

impl FromStr for Sensor {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        if let Ok(("", (_, x, _, y, _, beacon_x, _, beacon_y))) = tuple((
            tag("Sensor at x="),
            number,
            tag(", y="),
            number,
            tag(": closest beacon is at x="),
            number,
            tag(", y="),
            number,
        ))(line)
        {
            Ok(Sensor {
                position: Coordinate::new(x, y),
                beacon: Coordinate::new(beacon_x, beacon_y),
            })
        } else {
            Err(format!("Invalid sensor: {line}"))
        }
    }
}

fn number(input: &str) -> IResult<&str, i64> {
    let (input, number) = recognize(tuple((opt(tag("-")), digit1)))(input)?;
    Ok((input, number.parse().unwrap()))
}

#[cfg(test)]
mod test {
    use super::*;

    static EXAMPLE: &[u8] = b"
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    lazy_static! {
        static ref EXAMPLE_SENSORS: Vec<Sensor> =
            read_lines(EXAMPLE).filter_not_empty().parse().collect();
    }

    #[test]
    fn test_coordinates_without_beacon_on_row() {
        let sensor = Sensor {
            position: Coordinate::new(8, 7),
            beacon: Coordinate::new(2, 10),
        };

        let result = sensor.coordinates_without_beacon_on_row(10);

        assert_eq!(result, Some(3..=14));
    }

    #[test]
    fn test_input_merged_ranges_for_2_000_000() {
        let result = ranges_without_beacon_on_row(&SENSORS, 2_000_000);

        assert_eq!(result, vec![-609345..=1374834, 1374836..=4537988]);
    }

    #[test]
    fn part1_example() {
        let result = number_of_coordinates_without_beacon_on_row(&EXAMPLE_SENSORS, 10);

        assert_eq!(result, 26);
    }

    #[test]
    fn part2_example() {
        let result = find_missing_beacon_within_zone(&EXAMPLE_SENSORS, 0, 20);

        assert_eq!(result, Some(Coordinate::new(14, 11)));
    }
}
