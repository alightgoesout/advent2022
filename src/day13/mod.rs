use itertools::Itertools;
use lazy_static::lazy_static;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::IResult;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::input::{read_lines, FilterNotEmpty, ParseExt};
use crate::Solution;

mod input;

lazy_static! {
    static ref PACKETS: Vec<Packet> = read_lines(input::INPUT)
        .filter_not_empty()
        .parse()
        .collect();
}

pub struct Day13;

impl Solution for Day13 {
    fn day(&self) -> u8 {
        13
    }

    fn part_one(&self) -> String {
        format!(
            "Sum of indices of correctly ordered pairs: {}",
            sum_indices_of_correctly_ordered_pairs(
                &PACKETS.iter().cloned().tuples().collect::<Vec<_>>()
            ),
        )
    }

    fn part_two(&self) -> String {
        format!("Decoder key: {}", compute_decoder_key(PACKETS.clone()))
    }
}

fn sum_indices_of_correctly_ordered_pairs(packets: &[(Packet, Packet)]) -> usize {
    packets
        .iter()
        .enumerate()
        .filter_map(|(index, (p1, p2))| (p1 <= p2).then_some(index + 1))
        .sum()
}

fn compute_decoder_key(mut packets: Vec<Packet>) -> usize {
    let first_divider = "[[2]]".parse::<Packet>().unwrap();
    let second_divider = "[[6]]".parse::<Packet>().unwrap();
    packets.push(first_divider.clone());
    packets.push(second_divider.clone());
    packets.sort();

    let mut decoder_key = 1;

    for i in 1..=packets.len() {
        let current_packet = &packets[i - 1];
        if current_packet == &first_divider || current_packet == &second_divider {
            decoder_key *= i
        }
    }

    decoder_key
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Packet {
    List(Vec<Packet>),
    Integer(u32),
}

impl Display for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(packet) => write!(f, "{}", packet),
            Self::List(packets) => {
                write!(f, "[")?;
                if !packets.is_empty() {
                    write!(f, "{}", packets[0])?;
                }
                for i in 1..packets.len() {
                    write!(f, ",{}", packets[i])?;
                }
                write!(f, "]")
            }
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Integer(i1), Self::Integer(i2)) => i1.cmp(i2),
            (Self::List(l1), Self::List(l2)) => compare(l1, l2),
            (Self::List(list), Self::Integer(integer)) => compare(list, &[Self::Integer(*integer)]),
            (Self::Integer(integer), Self::List(list)) => compare(&[Self::Integer(*integer)], list),
        }
    }
}

fn compare(l1: &[Packet], l2: &[Packet]) -> Ordering {
    for i in 0..l1.len().min(l2.len()) {
        let comparison = l1[i].cmp(&l2[i]);
        if comparison != Ordering::Equal {
            return comparison;
        }
    }
    l1.len().cmp(&l2.len())
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Packet {
    type Err = String;

    fn from_str(packet: &str) -> Result<Self, Self::Err> {
        if let Ok(("", packet)) = parse_packet(packet) {
            Ok(packet)
        } else {
            Err(format!("Invalid packet data: {packet}"))
        }
    }
}

fn parse_packet(input: &str) -> IResult<&str, Packet> {
    alt((parse_list_packet, parse_integer_packet))(input)
}

fn parse_list_packet(input: &str) -> IResult<&str, Packet> {
    let (input, packets) =
        delimited(tag("["), separated_list0(tag(","), parse_packet), tag("]"))(input)?;
    Ok((input, Packet::List(packets)))
}

fn parse_integer_packet(input: &str) -> IResult<&str, Packet> {
    let (input, integer) = digit1(input)?;
    Ok((input, Packet::Integer(integer.parse().unwrap())))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_integer() {
        let packet = "1".parse::<Packet>();

        assert_eq!(packet, Ok(Packet::Integer(1)));
    }

    #[test]
    fn parse_integer_list() {
        let packet = "[1,2]".parse::<Packet>();

        assert_eq!(
            packet,
            Ok(Packet::List(vec![Packet::Integer(1), Packet::Integer(2)])),
        );
    }

    #[test]
    fn parse_list_of_list() {
        let packet = "[[1],[2,3,4]]".parse::<Packet>();

        assert_eq!(
            packet,
            Ok(Packet::List(vec![
                Packet::List(vec![Packet::Integer(1)]),
                Packet::List(vec![
                    Packet::Integer(2),
                    Packet::Integer(3),
                    Packet::Integer(4)
                ]),
            ])),
        );
    }

    #[test]
    fn compare_first_example() {
        let left = "[1,1,3,1,1]".parse::<Packet>();
        let right = "[1,1,5,1,1]".parse::<Packet>();

        assert!(left < right);
    }

    #[test]
    fn compare_second_example() {
        let left = "[[1],[2,3,4]]".parse::<Packet>();
        let right = "[[1],4]".parse::<Packet>();

        assert!(left < right);
    }

    #[test]
    fn compare_third_example() {
        let left = "[9]".parse::<Packet>();
        let right = "[[8,7,6]]".parse::<Packet>();

        assert!(left > right);
    }

    #[test]
    fn compare_test() {
        let left = "[1,[2,[3,[4,[5,6,0]]]],8,9]".parse::<Packet>();
        let right = "[[1],4]".parse::<Packet>();

        assert!(left < right);
    }

    #[test]
    fn sort_example() {
        let mut packets = read_lines(EXAMPLE)
            .filter_not_empty()
            .parse::<Packet>()
            .collect::<Vec<_>>();
        packets.sort();

        let result = packets.iter().map(Packet::to_string).join("\n");

        assert_eq!(
            &result,
            "[]
[[]]
[[[]]]
[1,1,3,1,1]
[1,1,5,1,1]
[[1],[2,3,4]]
[1,[2,[3,[4,[5,6,0]]]],8,9]
[1,[2,[3,[4,[5,6,7]]]],8,9]
[[1],4]
[3]
[[4,4],4,4]
[[4,4],4,4,4]
[7,7,7]
[7,7,7,7]
[[8,7,6]]
[9]",
        )
    }

    #[test]
    fn part2_example() {
        let packets = read_lines(EXAMPLE)
            .filter_not_empty()
            .parse::<Packet>()
            .collect::<Vec<_>>();

        let decoder_key = compute_decoder_key(packets);

        assert_eq!(decoder_key, 140);
    }

    static EXAMPLE: &[u8] = b"
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
";
}
