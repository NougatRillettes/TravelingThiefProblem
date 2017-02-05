
use std::io::prelude::*;

mod instance;

fn main() {
    let stdin = std::io::stdin();
    let inst = instance::read_instance(stdin.lock());
    println!("{:?}", inst);
}
