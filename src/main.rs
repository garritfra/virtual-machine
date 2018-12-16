pub mod parser;
pub mod stack;
pub mod vm;

use std::env;
use vm::VM;

fn main() {
    let exe_name = env::args().next().unwrap();
    match env::args().nth(1) {
        Some(filename) => run_with_file(filename),
        None => println!("Usage: {:?} <Filename>", exe_name),
    }
}

fn run_with_file(filename: String) {
    let instructions = parser::parse_file(&filename).unwrap();
    let mut vm = VM::new();
    vm.load_instructions(instructions);
    vm.run();
}
