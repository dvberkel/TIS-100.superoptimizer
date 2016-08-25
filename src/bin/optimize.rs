extern crate yaml_rust;
extern crate tis_100_superoptimizer;

use std::env;
use std::fs::File;
use std::io::Read;
use yaml_rust::YamlLoader;

use tis_100_superoptimizer::TIS_100::Node;
use tis_100_superoptimizer::TIS_100::Ports::Port;
use tis_100_superoptimizer::optimizer::{Config, optimize};

fn to_vec(yaml_doc: Option<&Vec<yaml_rust::Yaml>>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![];
    for value in yaml_doc.unwrap() {
        result.push(value.as_i64().unwrap() as i32);
    }
    result
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() != 2 {
        println!("provide a file to optimize");
        return;
    }

    let mut data = String::new();
    let mut f = File::open(&arguments[1]).unwrap();
    f.read_to_string(&mut data).unwrap();

    let docs = YamlLoader::load_from_str(&data[..]).unwrap();
    let doc = &docs[0];

    let node: Node = Node::new().set_up(Port::new(to_vec(doc["input"].as_vec())));
    let expected_output: Vec<i32> = to_vec(doc["output"].as_vec());
    let config: Config = Config::new(10, 3);

    match optimize(node, expected_output, config) {
        Some(program) => println!("{:?}", program),
        _ => println!("Could not find a program within the bounds"),
    }
}
