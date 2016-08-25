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
            digits_of(self.current, 33)
            .iter()
            .map(|digit| instruction_map(*digit))
            .collect();
        self.current += 1;
        Some(Program(instructions))
    }
}

fn digits_of(mut n: u32, base: u32) -> Vec<u32> {
    let mut digits: Vec<u32> = vec![];
    while n > 0 {
        digits.push(n.rem(base));
        n = n.div(base);
    }
    digits
}

fn instruction_map(index: u32) -> Instruction {
    match index % 33 {
        0 => Instruction::NOP,
        1 => Instruction::SWP,
        2 => Instruction::SAV,
        3 => Instruction::ADD(Source::Port),
        4 => Instruction::ADD(Source::Register(Register::NIL)),
        5 => Instruction::ADD(Source::Register(Register::ACC)),
        6 => Instruction::ADD(Source::Literal(0)),
        7 => Instruction::ADD(Source::Literal(1)),
        8 => Instruction::ADD(Source::Literal(-1)),
        9 => Instruction::SUB(Source::Port),
        10 => Instruction::SUB(Source::Register(Register::NIL)),
        11 => Instruction::SUB(Source::Register(Register::ACC)),
        12 => Instruction::SUB(Source::Literal(0)),
        13 => Instruction::SUB(Source::Literal(1)),
        14 => Instruction::SUB(Source::Literal(-1)),
        15 => Instruction::MOV(Source::Port, Destination::Port),
        16 => Instruction::MOV(Source::Register(Register::NIL), Destination::Port),
        17 => Instruction::MOV(Source::Register(Register::ACC), Destination::Port),
        18 => Instruction::MOV(Source::Literal(0), Destination::Port),
        19 => Instruction::MOV(Source::Literal(1), Destination::Port),
        20 => Instruction::MOV(Source::Literal(-1), Destination::Port),
        21 => Instruction::MOV(Source::Port, Destination::Register(Register::NIL)),
        22 => Instruction::MOV(Source::Register(Register::NIL), Destination::Register(Register::NIL)),
        23 => Instruction::MOV(Source::Register(Register::ACC), Destination::Register(Register::NIL)),
        24 => Instruction::MOV(Source::Literal(0), Destination::Register(Register::NIL)),
        25 => Instruction::MOV(Source::Literal(1), Destination::Register(Register::NIL)),
        26 => Instruction::MOV(Source::Literal(-1), Destination::Register(Register::NIL)),
        27 => Instruction::MOV(Source::Port, Destination::Register(Register::ACC)),
        28 => Instruction::MOV(Source::Register(Register::NIL), Destination::Register(Register::ACC)),
        29 => Instruction::MOV(Source::Register(Register::ACC), Destination::Register(Register::ACC)),
        30 => Instruction::MOV(Source::Literal(0), Destination::Register(Register::ACC)),
        31 => Instruction::MOV(Source::Literal(1), Destination::Register(Register::ACC)),
        32 => Instruction::MOV(Source::Literal(-1), Destination::Register(Register::ACC)),
        _ => Instruction::NOP,
    }
}

/// Iterator over Destination
pub struct DestinationIterator {
    current: i32,
    register_iterator: RegisterIterator,
}

impl DestinationIterator {
    /// create a DestinationIterator
    fn new() -> DestinationIterator {
        DestinationIterator {
            current: 0,
            register_iterator: RegisterIterator::new(),
        }
    }
}

impl Iterator for DestinationIterator {
    type Item = Destination;

    fn next(&mut self) -> Option<Destination> {
        let next_destination = match self.current {
            0 => Some(Destination::Port),
            _ => {
                self.register_iterator.next().map(|register| Destination::Register(register))
            }
        };
        self.current += 1;
        next_destination
    }
}

/// Iterator over Registers
pub struct RegisterIterator {
    current: u32,
}

impl RegisterIterator {
    fn new() -> RegisterIterator {
        RegisterIterator { current: 0 }
    }
}

impl Iterator for RegisterIterator {
    type Item = Register;

    fn next(&mut self) -> Option<Register> {
        let result = match self.current {
            0 => Some(Register::NIL),
            1 => Some(Register::ACC),
            _ => None
        };
        self.current += 1;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::super::TIS_100::{Program,Instruction,Source,Destination,Register};
    use super::*;

    #[test]
    fn should_iterator_over_destinations() {
        let destinations: Vec<Destination> = DestinationIterator::new().collect();

        assert_eq!(Destination::Port, destinations[0]);
        assert_eq!(Destination::Register(Register::NIL), destinations[1]);
        assert_eq!(Destination::Register(Register::ACC), destinations[2]);
    }

    #[test]
    fn should_iterator_over_registers() {
        let registers: Vec<Register> = RegisterIterator::new().collect();

        assert_eq!(Register::NIL, registers[0]);
        assert_eq!(Register::ACC, registers[1]);
    }

    #[test]
    fn should_iterate_over_programs() {
        let programs: Vec<Program> = ProgramIterator::new().take(34).collect();

        assert_eq!(Program(vec![]), programs[0]);
        assert_eq!(Program(vec![Instruction::SWP]), programs[1]);
        assert_eq!(Program(vec![Instruction::SAV]), programs[2]);
        assert_eq!(Program(vec![
            Instruction::MOV(
                Source::Literal(-1),
                Destination::Register(Register::ACC)),
        ]), programs[32]);
        assert_eq!(Program(vec![
            Instruction::NOP,
            Instruction::SWP,
        ]), programs[33])
    }
}
