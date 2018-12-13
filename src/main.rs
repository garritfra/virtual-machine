fn main() {
    let instructions = vec![0x00, 0x00, 0xff];
    let mut vm = VM::new();
    vm.load_instructions(instructions);
    vm.run();
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
    running: bool,
}

impl VM {
    fn new() -> VM {
        VM {
            ip: 0,
            instructions: vec![],
            stack: Stack::new(1024),
            running: false,
        }
    }

    fn load_instructions(&mut self, instructions: Vec<u8>) {
        self.instructions = instructions;
    }

    fn fetch(&mut self) {
        self.ip += 1;
    }

    fn run(&mut self) {
        self.running = true;
        while self.running {
            match self.instructions[(self.ip) as usize] {
                0x00 => self.op_nop(),
                0xff => self.op_halt(),
                _ => self.op_halt(),
            }
            self.fetch();
        }
    }

    fn op_nop(&mut self) {
        println!("NO OP");
    }

    fn op_halt(&mut self) {
        println!("HALT");
        self.running = false;
    }
}
