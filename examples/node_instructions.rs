extern crate tis_100_superoptimizer;

use tis_100_superoptimizer::TIS_100::{Node,Instruction,Source,Destination};

fn main() {
    let node: Node = Node::new();
    let last: Node = node
        .execute(Instruction::MOV(Source::Literal(1), Destination::ACC))
        .execute(Instruction::ADD(Source::ACC));

    println!("2 times 1 equals {}", last.acc);
}
