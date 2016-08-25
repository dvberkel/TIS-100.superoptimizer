//! The `optimizer` module contains iterators that allow a user to iterate over
//! `Program`s in order of increasing content
//!
//! The content is a measure of how complex a Program is.

use std::ops::{Rem, Div};
use super::TIS_100::{Program, Instruction, Source, Destination, Register};

/// Allows one to determine the content of certain constructs, e.g. `Program`s
/// or `Instruction`s
pub trait Content {
    /// the content of the construct
    fn content(&self) -> u32;
}

impl Content for Program {
    fn content(&self) -> u32 {
        let Program(ref instructions) = *self;

        instructions.iter().fold(0 as u32, |sum, instruction| sum + instruction.content())
    }
}

impl Content for Instruction {
    fn content(&self) -> u32 {
        match *self {
            Instruction::NOP => 1,
            Instruction::MOV(ref source, ref destination) => 1 + source.content() + destination.content(),
            Instruction::SWP => 1,
            Instruction::SAV => 1,
            Instruction::ADD(ref source) => 1 + source.content(),
            Instruction::SUB(ref source) => 1 + source.content(),
        }
    }
}

impl Content for Source {
    fn content(&self) -> u32 {
        match *self {
            Source::Literal(value) => 1 + (value.abs() as u32),
            _ => 1,
        }
    }
}

impl Content for Destination {
    fn content(&self) -> u32 {
        1
    }
}

/// Iterator over Programs
pub struct ProgramIterator {
    current: u32,
}

impl ProgramIterator {
    /// Create a `ProgramIterator`
    pub fn new() -> ProgramIterator {
        ProgramIterator { current: 0 }
    }
}

impl Iterator for ProgramIterator {
    type Item = Program;

    fn next(&mut self) -> Option<Program> {
        let instructions: Vec<Instruction> =
            digits_of(self.current, 18)
            .iter()
            .map(|digit| instruction_map(*digit))
            .collect();
        self.current += 1;
        Some(Program(instructions))
    }
}

fn digits_of(mut n: u32, base: u32) -> Vec<u32> {
    let mut digits: Vec<u32> = vec![];
    loop {
        digits.push(n.rem(base));
        n = n.div(base);
        if n == 0 { break; }
    }
    digits
}

fn instruction_map(index: u32) -> Instruction {
    match index.rem(18) {
        0 => Instruction::SWP,
        1 => Instruction::SAV,
        2 => Instruction::ADD(Source::Port),
        3 => Instruction::ADD(Source::Register(Register::ACC)),
        4 => Instruction::ADD(Source::Literal(1)),
        5 => Instruction::SUB(Source::Port),
        6 => Instruction::SUB(Source::Register(Register::ACC)),
        7 => Instruction::SUB(Source::Literal(1)),
        8 => Instruction::MOV(Source::Port, Destination::Port),
        9 => Instruction::MOV(Source::Register(Register::ACC), Destination::Port),
        10 => Instruction::MOV(Source::Literal(0), Destination::Port),
        11 => Instruction::MOV(Source::Literal(1), Destination::Port),
        12 => Instruction::MOV(Source::Literal(-1), Destination::Port),
        13 => Instruction::MOV(Source::Port, Destination::Register(Register::ACC)),
        14 => Instruction::MOV(Source::Register(Register::ACC), Destination::Register(Register::ACC)),
        15 => Instruction::MOV(Source::Literal(0), Destination::Register(Register::ACC)),
        16 => Instruction::MOV(Source::Literal(1), Destination::Register(Register::ACC)),
        17 => Instruction::MOV(Source::Literal(-1), Destination::Register(Register::ACC)),
        _ => Instruction::NOP,
    }
}

#[cfg(test)]
mod tests {
    use super::super::TIS_100::{Program,Instruction,Source,Destination,Register};
    use super::*;

    #[test]
    fn should_iterate_over_programs() {
        let programs: Vec<Program> = ProgramIterator::new().take(20).collect();

        assert_eq!(Program(vec![Instruction::SWP]), programs[0]);
        assert_eq!(Program(vec![Instruction::SAV]), programs[1]);
        assert_eq!(Program(vec![
            Instruction::MOV(
                Source::Literal(-1),
                Destination::Register(Register::ACC)),
        ]), programs[17]);
        assert_eq!(Program(vec![
            Instruction::SWP,
            Instruction::SAV,
        ]), programs[18])
    }
}
