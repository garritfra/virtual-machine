pub mod parser;
pub mod stack;
pub mod vm;

use std::env;
use vm::VM;

/// Should run with a filename as argument.
fn main() {
    let exe_name = env::args().next().unwrap();
    match env::args().nth(1) {
        Some(filename) => run_with_file(filename),
        None => println!("Usage: {:?} <Filename>", exe_name),
    }
}
fn run_with_file(filename: String) {
    let mut vm = VM::new();
    let instructions = parser::parse_file(&filename);
    vm.load_instructions(instructions);

    vm.run();
}
