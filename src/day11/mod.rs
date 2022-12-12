use itertools::Itertools;
use lazy_static::lazy_static;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{multispace0, multispace1};
use nom::character::is_digit;
use nom::multi::{separated_list0, separated_list1};
use nom::sequence::tuple;
use nom::IResult;
use std::fmt::Debug;
use std::str::FromStr;

use crate::Solution;

lazy_static! {
    static ref MONKEYS: Vec<Monkey> = parse_monkeys(input::INPUT);
}

mod input;

pub struct Day11;

impl Solution for Day11 {
    fn day(&self) -> u8 {
        11
    }

    fn part_one(&self) -> String {
        let mut monkeys = MONKEYS.clone();
        format!(
            "Level of monkey business after 20 rounds: {}",
            compute_monkey_business(&mut monkeys, 20, true)
        )
    }

    fn part_two(&self) -> String {
        let mut monkeys = MONKEYS.clone();
        format!(
            "Level of monkey business after 10 000 rounds: {}",
            compute_monkey_business(&mut monkeys, 10_000, false)
        )
    }
}

fn compute_monkey_business(
    monkeys: &mut [Monkey],
    rounds: usize,
    worry_level_reduction: bool,
) -> usize {
    let mut inspections = vec![0; monkeys.len()];

    for _ in 0..rounds {
        let new_inspections = play_round(monkeys, worry_level_reduction);
        inspections = inspections
            .into_iter()
            .zip(new_inspections.into_iter())
            .map(|(a, b)| a + b)
            .collect();
    }

    inspections.iter().sorted().rev().take(2).product::<usize>()
}

fn play_round(monkeys: &mut [Monkey], worry_level_reduction: bool) -> Vec<usize> {
    let mut result = Vec::new();
    let mut items = monkeys
        .iter()
        .map(|monkey| monkey.items.clone())
        .collect::<Vec<_>>();

    for monkey in &*monkeys {
        let mut monkey_items = Vec::new();
        monkey_items.append(&mut items[monkey.number]);
        result.push(monkey_items.len());
        for worry_level in monkey_items {
            let mut new_worry_level = monkey.operation.apply(worry_level);

            if worry_level_reduction {
                new_worry_level /= 3;
            } else {
                new_worry_level %= monkeys
                    .iter()
                    .map(|monkey| monkey.divisible_test)
                    .product::<u64>();
            }

            let target = if new_worry_level % monkey.divisible_test == 0 {
                monkey.on_true_monkey
            } else {
                monkey.on_false_monkey
            };

            items[target].push(new_worry_level);
        }
    }

    for monkey in monkeys {
        monkey.items = items[monkey.number].clone();
    }

    result
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Monkey {
    number: usize,
    items: Vec<u64>,
    operation: Operation,
    divisible_test: u64,
    on_true_monkey: usize,
    on_false_monkey: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

impl Operation {
    fn apply(&self, worry_level: u64) -> u64 {
        match self {
            Self::Add(operand) => worry_level + *operand,
            Self::Multiply(operand) => worry_level * *operand,
            Self::Square => worry_level * worry_level,
        }
    }
}

fn number<T>(input: &[u8]) -> IResult<&[u8], T>
where
    T: FromStr,
    T::Err: Debug,
{
    let (input, number) = take_while1(is_digit)(input)?;
    Ok((input, to_number(number)))
}

fn monkey_number(input: &[u8]) -> IResult<&[u8], usize> {
    let (input, (_, _, number, _)) = tuple((tag("Monkey"), multispace1, number, tag(":")))(input)?;
    Ok((input, number))
}

fn items(input: &[u8]) -> IResult<&[u8], Vec<u64>> {
    let (input, (_, items)) =
        tuple((tag("Starting items: "), separated_list1(tag(", "), number)))(input)?;
    Ok((input, items))
}

fn operation(input: &[u8]) -> IResult<&[u8], Operation> {
    let (input, (_, operator, _, operand)) = tuple((
        tag("Operation: new = old "),
        alt((tag("+"), tag("*"))),
        multispace1,
        alt((take_while1(is_digit), tag("old"))),
    ))(input)?;
    let operation = if operator == b"+" {
        Operation::Add(to_number(operand))
    } else if operand == b"old".as_slice() {
        Operation::Square
    } else {
        Operation::Multiply(to_number(operand))
    };
    Ok((input, operation))
}

fn divisible_test(input: &[u8]) -> IResult<&[u8], u64> {
    let (input, (_, number)) = tuple((tag("Test: divisible by "), number))(input)?;
    Ok((input, number))
}

fn monkey_target(input: &[u8]) -> IResult<&[u8], usize> {
    let (input, (_, number)) = tuple((
        alt((
            tag("If true: throw to monkey "),
            tag("If false: throw to monkey "),
        )),
        number,
    ))(input)?;
    Ok((input, number))
}

fn monkey(input: &[u8]) -> IResult<&[u8], Monkey> {
    let (
        input,
        (number, _, items, _, operation, _, divisible_test, _, on_true_monkey, _, on_false_monkey),
    ) = tuple((
        monkey_number,
        multispace1,
        items,
        multispace1,
        operation,
        multispace1,
        divisible_test,
        multispace1,
        monkey_target,
        multispace1,
        monkey_target,
    ))(input)?;

    Ok((
        input,
        Monkey {
            number,
            items,
            operation,
            divisible_test,
            on_true_monkey,
            on_false_monkey,
        },
    ))
}

fn parse_monkeys(input: &[u8]) -> Vec<Monkey> {
    let (_, (_, monkeys)) =
        tuple((multispace0, separated_list0(multispace1, monkey)))(input).unwrap();
    monkeys
}

fn to_number<T>(input: &[u8]) -> T
where
    T: FromStr,
    T::Err: Debug,
{
    String::from_utf8_lossy(input).parse::<T>().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::{assert_eq, vec};

    static EXAMPLE: &[u8] = b"
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    fn parse_example_first_monkey() {
        let result = monkey(
            b"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3",
        );

        assert_eq!(
            result,
            Ok((
                b"".as_slice(),
                Monkey {
                    number: 0,
                    items: vec![79, 98],
                    operation: Operation::Multiply(19),
                    divisible_test: 23,
                    on_true_monkey: 2,
                    on_false_monkey: 3,
                }
            ))
        )
    }

    #[test]
    fn parse_example() {
        let monkeys = parse_monkeys(EXAMPLE);

        assert_eq!(
            monkeys,
            vec![
                Monkey {
                    number: 0,
                    items: vec![79, 98],
                    operation: Operation::Multiply(19),
                    divisible_test: 23,
                    on_true_monkey: 2,
                    on_false_monkey: 3,
                },
                Monkey {
                    number: 1,
                    items: vec![54, 65, 75, 74],
                    operation: Operation::Add(6),
                    divisible_test: 19,
                    on_true_monkey: 2,
                    on_false_monkey: 0,
                },
                Monkey {
                    number: 2,
                    items: vec![79, 60, 97],
                    operation: Operation::Square,
                    divisible_test: 13,
                    on_true_monkey: 1,
                    on_false_monkey: 3,
                },
                Monkey {
                    number: 3,
                    items: vec![74],
                    operation: Operation::Add(3),
                    divisible_test: 17,
                    on_true_monkey: 0,
                    on_false_monkey: 1,
                },
            ],
        )
    }

    #[test]
    fn example_first_round() {
        let mut monkeys = parse_monkeys(EXAMPLE);

        play_round(&mut monkeys, true);

        assert_eq!(monkeys[0].items, vec![20, 23, 27, 26]);
        assert_eq!(monkeys[1].items, vec![2080, 25, 167, 207, 401, 1046]);
        assert_eq!(monkeys[2].items, vec![]);
        assert_eq!(monkeys[3].items, vec![]);
    }
}
