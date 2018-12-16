mod parser;
mod stack;
mod vm;

use vm::VM;

fn main() {
    let instructions = parser::parse_file("./example.dasm".to_string()).unwrap();
    let mut vm = VM::new();
    vm.load_instructions(instructions);
    vm.run();
}
