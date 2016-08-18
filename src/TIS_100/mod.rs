//! The Tessellated Intelligence System is a
//!
//! > massively parallel computer architecture comprised of non-uniformly
//! > interconnected heterogeneous nodes.
//!
//! The manual can be found [online](https://www.vidarholen.net/contents/junk/files/TIS-100%20Reference%20Manual.pdf).
//! Another useful resource is the unofficial [TIS-100 Hackers Guide](https://alandesmet.github.io/TIS-100-Hackers-Guide/).
//!
//! # Examples
//!
//! ```
//! let node: Node = Node:new();
//! let last: Node = node
//!                  .execute(Instruction::Add(Source::Literal(1)))
//!                  .execute(Instruction::Add(Source::ACC));
//! println!("2 times 1 equals {}", last.acc);
//! ```

/// A `Node` models the basic execution node in TIS-100. You change a node state
/// by executing an `Instruction` on it.
#[derive(Debug)]
pub struct Node {
    pub acc: i32,
    bac: i32
}

impl PartialEq for Node {
    /// Node are equal when they have the same internal state
    fn eq(&self, other: &Node) -> bool {
        self.acc == other.acc && self.bac == other.bac
    }
}

impl Eq for Node {}

/// `Instruction`s are executed by a `Node`.
pub enum Instruction {
    NOP,
    MOV(Source, Destination),
    SWP,
    SAV,
    ADD(Source),
    SUB(Source),
}

/// Source are either ports, registers or literals
pub enum Source {
    NIL,
    ACC,
    Literal(i32)
}


/// Destination are either ports or registers
pub enum Destination {
    NIL,
    ACC
}

impl Node {
    pub fn new() -> Node {
        Node { acc: 0, bac: 0 }
    }

    pub fn with(acc: i32, bac: i32) -> Node {
        Node { acc: acc, bac: bac }
    }

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
        Node { acc: self.acc, bac: self.bac }
    }

    fn mov(&self, source: Source, destination: Destination) -> Node {
        let value: i32 = match source {
            Source::NIL => 0,
            Source::ACC => self.acc,
            Source::Literal(value) => value
        };

        self.move_value(value, destination)
    }

    fn move_value(&self, value: i32, destination: Destination) -> Node {
        match destination {
            Destination::ACC => Node { acc: value, bac: self.bac },
            _ => self.nop()
        }
    }

    fn swap(&self) -> Node {
        Node { acc: self.bac, bac: self.acc }
    }

    fn save(&self) -> Node {
        Node { acc: self.acc, bac: self.acc }
    }

    fn add(&self, source: Source) -> Node{
        let value: i32 = match source {
            Source::NIL => 0,
            Source::ACC => self.acc,
            Source::Literal(value) => value
        };

        self.add_value(value)
    }

    fn add_value(&self, value: i32) -> Node {
        Node { acc: self.acc + value, bac: self.bac }
    }

    fn subtract(&self, source: Source) -> Node {
        let value: i32 = match source {
            Source::NIL => 0,
            Source::ACC => self.acc,
            Source::Literal(value) => value
        };

        self.subtract_value(value)
    }

    fn subtract_value(&self, value: i32) -> Node {
        Node { acc: self.acc - value, bac: self.bac }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let node: Node = Node::with(1, 0);
        let instruction: Instruction = Instruction::NOP;

        let next: Node = node.execute(instruction);

        assert_eq!(Node::with(1, 0), next);
    }

    #[test]
    fn node_should_execute_MOV_from_NIL_to_NIL_correctly() {
        let node: Node = Node::with(1, 0);
        let instruction: Instruction = Instruction::MOV(Source::NIL,
                                                        Destination::NIL);

        let next: Node = node.execute(instruction);

        assert_eq!(Node::with(1, 0), next);
    }

    #[test]
    fn node_should_execute_MOV_from_ACC_to_ACC_correctly() {
        let node: Node = Node::with(2, 1);
        let instruction: Instruction = Instruction::MOV(Source::ACC,
                                                        Destination::ACC);

        let next: Node = node.execute(instruction);

        assert_eq!(Node::with(2, 1), next);
    }

    #[test]
    fn node_should_execute_MOV_from_Literal_to_ACC_correctly() {
        let node: Node = Node::new();
        let instruction: Instruction = Instruction::MOV(Source::Literal(1),
                                                        Destination::ACC);

        let next: Node = node.execute(instruction);

        assert_eq!(Node::with(1, 0), next);
    }

    #[test]
    fn node_should_execute_SWP_correctly() {
        let node: Node = Node::with(1, 0);
        let instruction: Instruction = Instruction::SWP;

        let next: Node = node.execute(instruction);

        assert_eq!(Node::with(0, 1), next);
    }

    #[test]
    fn node_should_execute_SAV_correctly() {
        let node: Node = Node::with(1,0);
        let instruction: Instruction = Instruction::SAV;

        let next: Node = node.execute(instruction);

        assert_eq!(Node::with(1, 1), next);
    }

    #[test]
    fn node_should_execute_ADD_from_Literal_correctly() {
        let node: Node = Node::with(1, 0);
        let instruction: Instruction = Instruction::ADD(Source::Literal(1));

        let next: Node = node.execute(instruction);

        assert_eq!(Node::with(2, 0), next);
    }

    #[test]
    fn node_should_execute_SUB_from_Literal_correctly() {
        let node: Node = Node::with(2, 0);
        let instruction: Instruction = Instruction::SUB(Source::Literal(1));

        let next: Node = node.execute(instruction);

        assert_eq!(Node::with(1, 0), next);
    }
}

