extern crate tis_100_superoptimizer;

use tis_100_superoptimizer::TIS_100::Node;
use tis_100_superoptimizer::TIS_100::Ports::Port;
use tis_100_superoptimizer::optimizer::{Config, optimize};

fn main() {
    let node: Node = Node::new().set_up(Port::new(vec![0, 1, 2, 3]));
    let expected_output: Vec<i32> = vec![0, 0, 0, 0];
    let config: Config = Config::new(10, 3);

    match optimize(node, expected_output, config) {
        Some(program) => println!("{:?}", program),
        _ => assert!(false)
    }
}
