extern crate tis_100_superoptimizer;

use tis_100_superoptimizer::TIS_100::{Node,Program,Instruction,Cycle,Source,Destination,Register};

fn main(){
    let program: Program = Program(vec![
        Instruction::MOV(Source::Literal(1), Destination::Register(Register::ACC)),
        Instruction::ADD(Source::Register(Register::ACC)),
        Instruction::ADD(Source::Register(Register::ACC)),
        Instruction::ADD(Source::Register(Register::ACC)),
        Instruction::ADD(Source::Register(Register::ACC)),
    ]);
    let node: Node = Node::new().load(program);

    match node.run(Cycle::Indefinetly) {
        Ok(result) => println!("2^4 equals {}", result.acc),
        _ => println!("Well, that was unexpected"),
    }
}
