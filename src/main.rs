mod stack;
mod vm;

use vm::VM;

fn main() {
    let instructions = vec![0x01, 0x10, 0x01, 0x11, 0x03, 0x02, 0xff];
    let mut vm = VM::new();
    vm.load_instructions(instructions);
    vm.run();
}
