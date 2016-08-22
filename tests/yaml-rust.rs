extern crate yaml_rust;

use std::fs::File;
use std::io::Read;
use yaml_rust::{YamlLoader};

#[test]
fn should_load_yaml_from_string() {
    let s = "
input:
  - 0
  - 1
  - 2
  - 3
output:
  - 1
  - 5
";
    let docs = YamlLoader::load_from_str(s).unwrap();
    let doc = &docs[0];

    assert_eq!(3 as i64, doc["input"][3].as_i64().unwrap());
}

#[test]
fn should_read_yaml_from_file() {
    let mut data = String::new();
    let mut f = File::open("tests/yaml-rust.yml").unwrap();
    f.read_to_string(&mut data).unwrap();

    let docs = YamlLoader::load_from_str(&data[..]).unwrap();
    let doc = &docs[0];

    assert_eq!(3 as i64, doc["input"][3].as_i64().unwrap());
}
