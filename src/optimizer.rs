//! The `optimizer` module contains iterators that allow a user to iterate over
//! `Program`s in order of increasing content
//!
//! The content is a measure of how complex a Program is.
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
    use super::super::TIS_100::{Destination,Register};
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
}
