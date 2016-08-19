//! The Tessellated Intelligence System is a
//!
//! > massively parallel computer architecture comprised of non-uniformly
//! > interconnected heterogeneous nodes.
//!
//! The manual can be found [online](https://www.vidarholen.net/contents/junk/files/TIS-100%20Reference%20Manual.pdf).
//! Another useful resource is the unofficial [TIS-100 Hackers Guide](https://alandesmet.github.io/TIS-100-Hackers-Guide/).
//!
//! # Examples
//! Here we take a node and execute some instructions and check if double 1 has
//! the expected result. 
//!
//! ```rust
//! extern crate tis_100_superoptimizer;
//!
//! use tis_100_superoptimizer::TIS_100::{Node,Instruction,Source,Destination,Register};
//!
//! fn main() {
//!     let node: Node = Node::new();
//!     let last: Node = node
//!         .execute(Instruction::MOV(Source::Literal(1), Destination::Register(Register::ACC)))
//!         .execute(Instruction::ADD(Source::Register(Register::ACC)));
//!
//!     assert_eq!(2, last.acc);
//! }
//! ```

/// A `Node` models the basic execution node in TIS-100. You change a node state
/// by executing an `Instruction` on it.
#[derive(Debug,PartialEq,Eq)]
pub struct Node {
    /// The accumulator for the basic execution node.
    pub acc: i32,
    bac: i32,
    pc: isize,
}

/// `Instruction`s are executed by a `Node`.
pub enum Instruction {
    /// Does nothing
    NOP,
    /// Moves value from `Source` to `Destination`
    MOV(Source, Destination),
    /// Swaps the value of the accumulator (acc) and the backup (bac) register
    SWP,
    /// Saves the value of the accumulator (acc) to the backup register
    SAV,
    /// Add value from `Source` to accumulator (acc), storing result in acc
    ADD(Source),
    /// Subtracts value from `Source` from accumulator (acc), storing result in acc
    SUB(Source),
}

/// `Source` are either ports, registers or literals
pub enum Source {
    /// A register
    Register(Register),
    /// A literal value
    Literal(i32)
}

/// Different types of Registers known in TIS-100
pub enum Register {
    /// the NIL register, reading from it provides with zero
    NIL,
    /// The accumulator
    ACC,
}


/// `Destination` are either ports or registers
pub enum Destination {
    /// A register
    Register(Register),
}

impl Node {
    /// Create a `Node` with defaults for accumulator and backup registers
    pub fn new() -> Node {
        Node { acc: 0, bac: 0, pc: 0 }
    }

    /// Create a `Node` from self with the program counter incremented
    pub fn increment_pc(&self) -> Node {
        Node { pc: self.pc + 1, .. *self }
    }

    /// Create a `Node` from self with a prescribed program counter value
    pub fn set_pc(&self, pc: isize) -> Node {
        Node { pc: pc, .. *self }
    }

    /// Create a `Node` from self with a prescribed accumulator register value
    pub fn set_acc(&self, acc: i32) -> Node {
        Node { acc: acc, .. *self }
    }

    /// Create a `Node` from self with a prescribed backup register value
    pub fn set_bac(&self, bac: i32) -> Node {
        Node { bac: bac, .. *self }
    }

    /// Execute the `instruction` on this `Node`. Returns a `Node` that reflects
    /// the changes the `instruction` would have on this `Node`.
    pub fn execute(&self, instruction: Instruction) -> Node {
        match instruction {
            Instruction::NOP => self.nop(),
            Instruction::MOV(source, destination) => self.mov(source, destination) ,
            Instruction::SWP => self.swap(),
            Instruction::SAV => self.save(),
            Instruction::ADD(source) => self.add(source),
            Instruction::SUB(source) => self.subtract(source),
        }
    }

    fn nop(&self) -> Node {
        self.increment_pc()
    }

    fn mov(&self, source: Source, destination: Destination) -> Node {
        let value: i32 = self.value_from(source);

        self.move_value(value, destination)
    }

    fn value_from(&self, source: Source) -> i32 {
        match source {
            Source::Register(Register::NIL) => 0,
            Source::Register(Register::ACC) => self.acc,
            Source::Literal(value) => value,
        }
    }

    fn move_value(&self, value: i32, destination: Destination) -> Node {
        match destination {
            Destination::Register(Register::ACC) => self.increment_pc().set_acc(value),
            _ => self.nop(),
        }
    }

    fn swap(&self) -> Node {
        let acc: i32 = self.acc;
        let bac: i32 = self.bac;

        self.increment_pc().set_acc(bac).set_bac(acc)
    }

    fn save(&self) -> Node {
        self.increment_pc().set_bac(self.acc)
    }

    fn add(&self, source: Source) -> Node{
        let value: i32 = self.value_from(source);

        self.add_value(value)
    }

    fn add_value(&self, value: i32) -> Node {
        self.increment_pc().set_acc(self.acc + value)
    }

    fn subtract(&self, source: Source) -> Node {
        let value: i32 = self.value_from(source);

        self.subtract_value(value)
    }

    fn subtract_value(&self, value: i32) -> Node {
        self.increment_pc().set_acc(self.acc - value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node_with(acc: i32, bac: i32, pc: isize) -> Node {
        Node::new().set_acc(acc).set_bac(bac).set_pc(pc)
    }

    #[test]
    fn node_should_start_with_accumulator_zero() {
        let node: Node = Node::new();

        assert_eq!(0, node.acc);
    }

    #[test]
    fn node_should_start_with_backup_zero() {
        let node: Node = Node::new();
        let instruction: Instruction = Instruction::SWP;

        let next: Node = node.execute(instruction);

        assert_eq!(0, next.acc);
    }

    #[test]
    fn node_should_execute_NOP_correctly() {
        let node: Node = Node::new().set_acc(1);
        let instruction: Instruction = Instruction::NOP;

        let next: Node = node.execute(instruction);

        assert_eq!(node_with(1, 0, 1), next);
    }

    #[test]
    fn node_should_execute_MOV_from_NIL_to_NIL_correctly() {
        let node: Node = Node::new().set_acc(1);
        let instruction: Instruction = Instruction::MOV(Source::Register(Register::NIL),
                                                        Destination::Register(Register::NIL));

        let next: Node = node.execute(instruction);

        assert_eq!(node_with(1, 0, 1), next);
    }

    #[test]
    fn node_should_execute_MOV_from_ACC_to_ACC_correctly() {
        let node: Node = Node::new().set_acc(2).set_bac(1);
        let instruction: Instruction = Instruction::MOV(Source::Register(Register::ACC),
                                                        Destination::Register(Register::ACC));

        let next: Node = node.execute(instruction);

        assert_eq!(node_with(2, 1, 1), next);
    }

    #[test]
    fn node_should_execute_MOV_from_Literal_to_ACC_correctly() {
        let node: Node = Node::new();
        let instruction: Instruction = Instruction::MOV(Source::Literal(1),
                                                        Destination::Register(Register::ACC));

        let next: Node = node.execute(instruction);

        assert_eq!(node_with(1, 0, 1), next);
    }

    #[test]
    fn node_should_execute_SWP_correctly() {
        let node: Node = Node::new().set_acc(1);
        let instruction: Instruction = Instruction::SWP;

        let next: Node = node.execute(instruction);

        assert_eq!(node_with(0, 1, 1), next);
    }

    #[test]
    fn node_should_execute_SAV_correctly() {
        let node: Node = Node::new().set_acc(1);
        let instruction: Instruction = Instruction::SAV;

        let next: Node = node.execute(instruction);

        assert_eq!(node_with(1, 1, 1), next);
    }

    #[test]
    fn node_should_execute_ADD_from_Literal_correctly() {
        let node: Node = Node::new().set_acc(1);
        let instruction: Instruction = Instruction::ADD(Source::Literal(1));

        let next: Node = node.execute(instruction);

        assert_eq!(node_with(2, 0, 1), next);
    }

    #[test]
    fn node_should_execute_SUB_from_Literal_correctly() {
        let node: Node = Node::new().set_acc(2);
        let instruction: Instruction = Instruction::SUB(Source::Literal(1));

        let next: Node = node.execute(instruction);

        assert_eq!(node_with(1, 0, 1), next);
    }
}

