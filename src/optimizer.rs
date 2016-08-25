//! Will find the shortest program to problem

use super::TIS_100::{Node,Program};
use super::iterator::{ProgramIterator};
use super::check::check;

/// Configuration for the optimize function
pub struct Config {
    /// The maximum allowed number of cycles per program
    pub maximum_cycle: u32,
    /// The maximum allowed program length
    pub maximum_program_length: usize,
}

impl Config {
    /// create a `Config` with prescribed maximum_cycle and maximum_program_length
    pub fn new(maximum_cycle: u32, maximum_program_length: usize) -> Config {
        Config {
            maximum_cycle: maximum_cycle,
            maximum_program_length: maximum_program_length,
        }
    }
}

/// Tries to find a `Program` that satisfies the context
pub fn optimize(node: Node, expected_output: Vec<i32>, config: Config) -> Option<Program> {
    for program in ProgramIterator::new() {
        if check(node.clone(), program.clone(), expected_output.clone(), config.maximum_cycle) {
            return Some(program.clone());
        }
        if length(&program) > config.maximum_program_length {break;}
    }
    None
}

fn length(program: &Program) -> usize {
    let Program(ref instructions) = *program;
    instructions.len()
}

#[cfg(test)]
mod tests {
    use super::super::TIS_100::{Node, Program, Instruction, Source, Destination};
    use super::super::TIS_100::Ports::Port;
    use super::*;

    #[test]
    fn should_find_simple_program_to_output_zero() {
        let node: Node = Node::new().set_up(Port::new(vec![0, 1, 2, 3]));
        let expected_output: Vec<i32> = vec![0, 0, 0, 0];
        let config: Config = Config::new(10,3);

        match optimize(node, expected_output, config) {
            Some(program) => assert_eq!(Program(vec![
                Instruction::MOV(Source::Literal(0), Destination::Port),
                Instruction::ADD(Source::Port),
            ]), program),
            _ => assert!(false)
        }

    }
}
