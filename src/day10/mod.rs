use lazy_static::lazy_static;
use std::str::FromStr;

use crate::input::{read_lines, FilterNotEmpty, ParseExt};
use crate::Solution;

mod input;

lazy_static! {
    static ref INSTRUCTIONS: Vec<Instruction> = read_lines(input::INPUT)
        .filter_not_empty()
        .parse()
        .collect();
}

pub struct Day10;

impl Solution for Day10 {
    fn day(&self) -> u8 {
        10
    }

    fn part_one(&self) -> String {
        let mut cpu = Cpu::default();
        let mut instructions = INSTRUCTIONS.iter().copied();
        format!(
            "Sum of the six signal strengths: {}",
            sum_six_signal_strengths(&mut cpu, &mut instructions),
        )
    }

    fn part_two(&self) -> String {
        let mut cpu = Cpu::default();
        let mut instructions = INSTRUCTIONS.iter().copied();
        format!(
            "Picture drawn on CRT:\n{}",
            cpu.execute_and_compute_picture(&mut instructions),
        )
    }
}

fn sum_six_signal_strengths<I: Iterator<Item = Instruction>>(
    cpu: &mut Cpu,
    instructions: &mut I,
) -> i32 {
    cpu.execute_and_compute_signal_strength(instructions, 20)
        + cpu.execute_and_compute_signal_strength(instructions, 40)
        + cpu.execute_and_compute_signal_strength(instructions, 40)
        + cpu.execute_and_compute_signal_strength(instructions, 40)
        + cpu.execute_and_compute_signal_strength(instructions, 40)
        + cpu.execute_and_compute_signal_strength(instructions, 40)
}

#[derive(Debug)]
struct Cpu {
    x_register: i32,
    cycles: usize,
    current_instruction: Option<(usize, Instruction)>,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            x_register: 1,
            cycles: 0,
            current_instruction: None,
        }
    }
}

impl Cpu {
    pub fn execute_and_compute_signal_strength<I: Iterator<Item = Instruction>>(
        &mut self,
        instructions: &mut I,
        cycles: usize,
    ) -> i32 {
        let mut signal_strength = 0;
        for _ in 0..cycles {
            let x_register = self.x_register;
            self.tick_with_instructions(instructions);
            signal_strength = self.cycles as i32 * x_register;
        }
        signal_strength
    }

    pub fn execute_and_compute_picture<I: Iterator<Item = Instruction>>(
        &mut self,
        instructions: &mut I,
    ) -> String {
        let mut picture = String::new();

        for i in 0..240 {
            let current_pixel = i % 40;
            let sprite_position = self.x_register;
            if (current_pixel - sprite_position).abs() <= 1 {
                picture.push('#');
            } else {
                picture.push(' ');
            }
            if current_pixel == 39 {
                picture.push('\n');
            }
            self.tick_with_instructions(instructions);
        }

        picture
    }

    fn tick_with_instructions<I: Iterator<Item = Instruction>>(&mut self, instructions: &mut I) {
        if self.is_idle() {
            if let Some(instruction) = instructions.next() {
                self.add_instruction(instruction);
            }
        }
        self.tick();
    }

    fn is_idle(&self) -> bool {
        self.current_instruction.is_none()
    }

    fn add_instruction(&mut self, instruction: Instruction) {
        self.current_instruction = Some((0, instruction));
    }

    fn tick(&mut self) {
        self.cycles += 1;
        if let Some((mut cycles, instruction)) = self.current_instruction {
            cycles += 1;
            if cycles == instruction.cycles() {
                if let Instruction::AddX(x) = instruction {
                    self.x_register += x;
                }
                self.current_instruction = None;
            } else {
                self.current_instruction = Some((cycles, instruction));
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Noop,
    AddX(i32),
}

impl Instruction {
    fn cycles(&self) -> usize {
        match self {
            Self::Noop => 1,
            Self::AddX(_) => 2,
        }
    }
}

const NOOP: &str = "noop";
const ADD_X: &str = "addx";

impl FromStr for Instruction {
    type Err = String;

    fn from_str(instruction: &str) -> Result<Self, Self::Err> {
        if instruction == NOOP {
            Ok(Instruction::Noop)
        } else if instruction.starts_with(ADD_X) {
            instruction[ADD_X.len() + 1..]
                .parse()
                .map(Instruction::AddX)
                .map_err(|_| format!("Invalid instruction: {instruction}"))
        } else {
            Err(format!("Invalid instruction: {instruction}"))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_small_example() {
        let mut cpu = Cpu::default();
        let mut instructions = read_lines(
            b"\
noop
addx 3
addx -5
"
            .as_slice(),
        )
        .filter_not_empty()
        .parse();

        cpu.execute_and_compute_signal_strength(&mut instructions, 5);

        assert_eq!(cpu.cycles, 5);
        assert_eq!(cpu.x_register, -1);
    }

    #[test]
    fn part1_large_example() {
        let mut cpu = Cpu::default();
        let mut instructions = read_lines(LARGE_EXAMPLE).filter_not_empty().parse();

        let result = sum_six_signal_strengths(&mut cpu, &mut instructions);

        assert_eq!(result, 13140);
    }

    #[test]
    fn part2_large_example() {
        let mut cpu = Cpu::default();
        let mut instructions = read_lines(LARGE_EXAMPLE).filter_not_empty().parse();

        let result = cpu.execute_and_compute_picture(&mut instructions);

        assert_eq!(
            &result,
            r"##  ##  ##  ##  ##  ##  ##  ##  ##  ##  
###   ###   ###   ###   ###   ###   ### 
####    ####    ####    ####    ####    
#####     #####     #####     #####     
######      ######      ######      ####
#######       #######       #######     
",
        );
    }

    static LARGE_EXAMPLE: &[u8] = b"
addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";
}
