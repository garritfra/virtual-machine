fn main() {
    let instructions = vec![0x00000001, 0x0000002];
    let mut vm = VM::new();
    vm.load_instructions(instructions);
    for instruction in vm.instructions {
        println!("{:x}", instruction);
    }
}

#[derive(Debug)]
struct Stack {
    top: u32,
    size: u32,
}
impl Stack {
    fn new(size: u32) -> Stack {
        Stack { top: 0, size: size }
    }
}

#[derive(Debug)]
struct VM {
    ip: u8,
    instructions: Vec<u8>,
    stack: Stack,
}

impl VM {
    fn new() -> VM {
        VM {
            ip: 0,
            instructions: vec![],
            stack: Stack::new(1024),
        }
    }

    fn load_instructions(&mut self, instructions: Vec<u8>) {
        self.instructions = instructions;
    }
}
