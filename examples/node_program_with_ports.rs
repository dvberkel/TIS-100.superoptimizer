extern crate tis_100_superoptimizer;

use tis_100_superoptimizer::TIS_100::{Node,Program,Instruction,Source,Destination,Register};
use tis_100_superoptimizer::TIS_100::Ports::Port;

fn main(){
    let input: i32 = 37;
    let program: Program = Program(vec![
        Instruction::MOV(Source::Port, Destination::Register(Register::ACC)),
        Instruction::ADD(Source::Register(Register::ACC)),
        Instruction::MOV(Source::Register(Register::ACC), Destination::Port),
    ]);
    let node: Node = Node::new().set_up(Port::new(vec![input])).load(program);

    match node.run() {
        Ok(result) => {
            let value = result.down.output[0];

            println!("2 times {} equals {}", input, value)
        },
        _ => println!("Well,that was unexpected"),
    }
}
