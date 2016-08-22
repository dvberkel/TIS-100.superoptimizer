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

pub mod Ports;

use std::fmt::{Debug,Formatter,Error};
use self::Ports::{Port, PortReadResult};

/// A `Node` models the basic execution node in TIS-100. You change a node state
/// by running `Program`s on it or executing an `Instruction` on it.
#[derive(Debug,PartialEq,Eq)]
pub struct Node {
    /// The accumulator for the basic execution node.
    pub acc: i32,
    /// The up `Port` used for reading
    pub up: Port,
    /// The down `Port` used for writing
    pub down: Port,
    bac: i32,
    pc: usize,
    program: Program,
}

/// A `Program` is a sequence of `Instruction`s
pub struct Program(pub Vec<Instruction>);

impl PartialEq for Program {
    fn eq(&self, other: &Program) -> bool {
        let Program(ref self_instructions) = *self;
        let Program(ref other_instructions) = *other;
        if self_instructions.len() == other_instructions.len() {
            for index in 0..(self_instructions.len()) {
                let ref self_instruction = self_instructions[index];
                let ref other_instruction = other_instructions[index];
                if self_instruction != other_instruction {
                    return false
                }
            }
            true
        } else {
            false
        }
    }
}

impl Eq for Program {}

impl Debug for Program {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let Program(ref instructions) = *self;
        write!(f, "[{}:", instructions.len()).unwrap();
        for instruction in instructions {
            write!(f, " {:?}", instruction).unwrap();
        }
        write!(f, "]")
    }
}

impl Clone for Program {
    fn clone(&self) -> Program {
        let mut clone_instructions: Vec<Instruction> = vec![];
        let Program(ref self_instructions) = *self;
        for ref mut instruction in self_instructions {
            clone_instructions.push(instruction.clone());
        }
        Program(clone_instructions)
    }
}

/// `Instruction`s are executed by a `Node`
#[derive(Debug,PartialEq,Eq,Clone)]
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
#[derive(Debug,PartialEq,Eq,Clone)]
pub enum Source {
    /// A port, will always be the up port
    Port,
    /// A register
    Register(Register),
    /// A literal value
    Literal(i32)
}

/// Different types of Registers known in TIS-100
#[derive(Debug,PartialEq,Eq,Clone)]
pub enum Register {
    /// the NIL register, reading from it provides with zero
    NIL,
    /// The accumulator
    ACC,
}


/// `Destination` are either ports or registers
#[derive(Debug,PartialEq,Eq,Clone)]
pub enum Destination {
    /// A Port, will always be the DOWN port
    Port,
    /// A register
    Register(Register),
}


/// The `Status` a `Program` run on a certain `Node`.
pub enum Status {
    /// a successful run of the program
    Successful(Node),
    /// an unsuccessful run of the program, because of a deadlock.
    Deadlock(Node),
}

impl Node {
    /// Create a `Node` with defaults for accumulator, backup registers, program counter and program
    pub fn new() -> Node {
        Node {
            acc: 0,
            bac: 0,
            pc: 0,
            program: Program(vec![]),
            up: Port::new(vec![]),
            down: Port::new(vec![]),
        }
    }

    /// Loads a program in this `Node`
    pub fn load(&self, program: Program) -> Node {
        Node { program: program.clone(), up: self.up.clone(), down: self.down.clone(), .. *self }
    }

    /// Run the loaded program, returning an calculation state
    pub fn run(&self) -> Status {
        let mut node = Node { program: self.program.clone(), up: self.up.clone(), down: self.down.clone(), .. *self };

        loop {
            match node.fetch_instruction() {
                Some(instruction) => {
                    node = node.execute(instruction);
                }
                None => {
                    break
                }
            }
        }

        Status::Successful(Node { program: node.program.clone(), .. node })
    }

    fn fetch_instruction(&self) -> Option<Instruction> {
        let Program(ref instructions) = self.program;
        if self.pc < instructions.len() {
            Some(instructions[self.pc].clone())
        } else {
            None
        }
    }

    /// Create a `Node` from self with the program counter incremented
    fn increment_pc(&self) -> Node {
        Node { pc: self.pc + 1, program: self.program.clone(), up: self.up.clone(), down: self.down.clone(), .. *self }
    }

    /// Create a `Node` from self with a prescribed program counter value
    fn set_pc(&self, pc: usize) -> Node {
        Node { pc: pc, program: self.program.clone(), up: self.up.clone(), down: self.down.clone(), .. *self }
    }

    /// Create a `Node` from self with a prescribed accumulator register value
    fn set_acc(&self, acc: i32) -> Node {
        Node { acc: acc, program: self.program.clone(), up: self.up.clone(), down: self.down.clone(), .. *self }
    }

    /// Create a `Node` from self with a prescribed backup register value
    fn set_bac(&self, bac: i32) -> Node {
        Node { bac: bac, program: self.program.clone(), up: self.up.clone(), down: self.down.clone(), .. *self }
    }

    /// Create a `Node` from self with a prescribed down port
    fn set_up(&self, up: Port) -> Node {
        Node { up: up, program: self.program.clone(), down: self.down.clone(), .. *self }
    }

    /// Create a `Node` from self with a prescribed down port
    fn set_down(&self, down: Port) -> Node {
        Node { down: down, program: self.program.clone(), up: self.up.clone(), .. *self }
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
        let (next_up_port, value): (Port,i32) = self.value_from(source);

        self.set_up(next_up_port).move_value(value, destination)
    }

    fn value_from(&self, source: Source) -> (Port,i32) {
        match source {
            Source::Port => {
                match self.up.read() {
                    PortReadResult::Success(next_up, value) => (next_up, value),
                    PortReadResult::Failure => (self.up.clone(), 3435), // TODO handle failure correctly
                }
            },
            Source::Register(Register::NIL) => (self.up.clone(),0),
            Source::Register(Register::ACC) => (self.up.clone(),self.acc),
            Source::Literal(value) => (self.up.clone(),value),
        }
    }

    fn move_value(&self, value: i32, destination: Destination) -> Node {
        match destination {
            Destination::Port => {
                let next_down = self.down.write(value);
                self.increment_pc().set_down(next_down)
            },
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
        let (next_up_port, value): (Port, i32) = self.value_from(source);

        self.set_up(next_up_port).add_value(value)
    }

    fn add_value(&self, value: i32) -> Node {
        self.increment_pc().set_acc(self.acc + value)
    }

    fn subtract(&self, source: Source) -> Node {
        let (next_up_port, value): (Port, i32) = self.value_from(source);

        self.set_up(next_up_port).subtract_value(value)
    }

    fn subtract_value(&self, value: i32) -> Node {
        self.increment_pc().set_acc(self.acc - value)
    }
}

#[cfg(test)]
mod tests {
    use super::Ports::*;
    use super::*;

    fn node_with(acc: i32, bac: i32, pc: usize, up: Vec<i32>, down: Vec<i32>) -> Node {
        Node::new()
            .set_acc(acc)
            .set_bac(bac)
            .set_pc(pc)
            .set_up(Port::with(up, vec![]))
            .set_down(Port::with(vec![], down))
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

        assert_eq!(node_with(1, 0, 1, vec![], vec![]), next);
    }

    #[test]
    fn node_should_execute_MOV_from_NIL_to_NIL_correctly() {
        let node: Node = Node::new().set_acc(1);
        let instruction: Instruction = Instruction::MOV(Source::Register(Register::NIL),
                                                        Destination::Register(Register::NIL));

        let next: Node = node.execute(instruction);

        assert_eq!(node_with(1, 0, 1, vec![], vec![]), next);
    }

    #[test]
    fn node_should_execute_MOV_from_ACC_to_ACC_correctly() {
        let node: Node = Node::new().set_acc(2).set_bac(1);
        let instruction: Instruction = Instruction::MOV(Source::Register(Register::ACC),
                                                        Destination::Register(Register::ACC));

        let next: Node = node.execute(instruction);

        assert_eq!(node_with(2, 1, 1, vec![], vec![]), next);
    }

    #[test]
    fn node_should_execute_MOV_from_Literal_to_ACC_correctly() {
        let node: Node = Node::new();
        let instruction: Instruction = Instruction::MOV(Source::Literal(1),
                                                        Destination::Register(Register::ACC));

        let next: Node = node.execute(instruction);

        assert_eq!(node_with(1, 0, 1, vec![], vec![]), next);
    }

    #[test]
    fn node_should_execute_MOV_from_Literal_to_Port_correctly() {
        let node: Node = Node::new();
        let instruction: Instruction = Instruction::MOV(Source::Literal(1),
                                                        Destination::Port);

        let next: Node = node.execute(instruction);

        assert_eq!(node_with(0, 0, 1, vec![], vec![1]), next);
    }

    #[test]
    fn node_should_execute_MOV_from_Port_to_ACC_correctly() {
        let node: Node = Node::new().set_up(Port::new(vec![1]));
        let instruction: Instruction = Instruction::MOV(Source::Port,
                                                        Destination::Register(Register::ACC));

        let next: Node = node.execute(instruction);

        assert_eq!(node_with(1, 0, 1, vec![], vec![]), next);
    }

    #[test]
    fn node_should_execute_SWP_correctly() {
        let node: Node = Node::new().set_acc(1);
        let instruction: Instruction = Instruction::SWP;

        let next: Node = node.execute(instruction);

        assert_eq!(node_with(0, 1, 1, vec![], vec![]), next);
    }

    #[test]
    fn node_should_execute_SAV_correctly() {
        let node: Node = Node::new().set_acc(1);
        let instruction: Instruction = Instruction::SAV;

        let next: Node = node.execute(instruction);

        assert_eq!(node_with(1, 1, 1, vec![], vec![]), next);
    }

    #[test]
    fn node_should_execute_ADD_from_Literal_correctly() {
        let node: Node = Node::new().set_acc(1);
        let instruction: Instruction = Instruction::ADD(Source::Literal(1));

        let next: Node = node.execute(instruction);

        assert_eq!(node_with(2, 0, 1, vec![], vec![]), next);
    }

    #[test]
    fn node_should_execute_SUB_from_Literal_correctly() {
        let node: Node = Node::new().set_acc(2);
        let instruction: Instruction = Instruction::SUB(Source::Literal(1));

        let next: Node = node.execute(instruction);

        assert_eq!(node_with(1, 0, 1, vec![], vec![]), next);
    }

    #[test]
    fn programs_should_differ_when_different_size() {
        assert_eq!(false, Program(vec![]) == Program(vec![Instruction::SAV]));
    }

    #[test]
    fn programs_should_differ_when_different_instructions() {
        assert_eq!(false, Program(vec![Instruction::SWP]) == Program(vec![Instruction::SAV]));
    }

    #[test]
    fn programs_should_equal_with_same_instructions() {
        assert_eq!(Program(vec![Instruction::SAV]), Program(vec![Instruction::SAV]));
    }

    #[test]
    fn node_should_execute_program_correctly() {
        let program: Program = Program(vec![// calculate 4 * source - 1
            Instruction::MOV(Source::Literal(1), Destination::Register(Register::ACC)),
            Instruction::ADD(Source::Register(Register::ACC)),
            Instruction::ADD(Source::Register(Register::ACC)),
            Instruction::SUB(Source::Literal(1))
        ]);
        let node: Node = Node::new().load(program);

        match node.run() {
            Status::Successful(_) => assert!(true),
            Status::Deadlock(_) => assert!(false),
        }
    }
}

