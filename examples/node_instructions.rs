extern crate tis_100_superoptimizer;

use tis_100_superoptimizer::TIS_100::{Node,Instruction,Source,Destination,Register};

fn main() {
    let node: Node = Node::new();
    let last: Node = node
        .execute(Instruction::MOV(Source::Literal(1), Destination::Register(Register::ACC)))
        .execute(Instruction::ADD(Source::Register(Register::ACC)));

    println!("2 times 1 equals {}", last.acc);
}
