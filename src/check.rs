//! The check module verifies if a certain `Program` when run on a specific
//! `Node` produces the expected result. I.e. Reads the input on `Source::Port`
//! and writes the correct sequence the `Destination::Port`.

use super::TIS_100::{Node, Program, Cycle};

/// Checks if `Node` when run with `Program` writes `expected_result` to `Destination::Port`
pub fn check(node: Node, program: Program, expected_result: Vec<i32>, maximum_cycle: u32) -> bool {
    let local_node: Node = node.load(program);

    match local_node.run(Cycle::Maximum(maximum_cycle)) {
        Ok(result_node) => {
            same(result_node.down.output, expected_result)
        },
        Err(_) => false,
    }
}

fn same<T: Eq>(left: Vec<T>, right: Vec<T>) -> bool {
    if left.len() == right.len() {
        for index in 0..(left.len()) {
            if left[index] != right[index] {
                return false;
            }
        }
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::super::TIS_100::{Node, Program, Instruction, Source, Destination, Register};
    use super::super::TIS_100::Ports::Port;
    use super::*;

    #[test]
    fn should_correctly_check_program() {
        let program: Program = Program(vec![
            Instruction::MOV(Source::Port, Destination::Register(Register::ACC)),
            Instruction::ADD(Source::Port),
            Instruction::MOV(Source::Register(Register::ACC), Destination::Port),
        ]);
        let node: Node = Node::new().set_up(Port::new(vec![0, 1, 2, 3]));

        assert!(check(node, program, vec![1, 5], 10))
    }
}
