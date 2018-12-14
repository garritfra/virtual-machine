fn main() {
    let instructions = vec![0x01, 0x10, 0x01, 0x11, 0x02, 0x02, 0xff];
    let mut vm = VM::new();
    vm.load_instructions(instructions);
    vm.run();
}

#[derive(Debug)]
struct Stack {
    top: u32,
    size: u32,
    stack: Vec<i32>,
}
impl Stack {
    fn new(size: u32) -> Stack {
        Stack {
            top: 0,
            size: size,
            stack: Vec::new(),
        }
    }

    fn push(&mut self, value: i32) {
        self.stack.push(value);
        self.top += 1;
    }

    fn pop(&mut self) -> Option<i32> {
        self.top -= 1;
        self.stack.pop()
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

    fn run(&mut self) {
        self.running = true;
        while self.running {
            match self.instructions[(self.ip) as usize] {
                0x00 => self.op_nop(),
                0x01 => self.op_push(),
                0x02 => self.op_print(),
                0xff => self.op_halt(),
                _ => self.op_halt(),
            }
        }
    }

    fn op_nop(&mut self) {
        self.ip += 1;
    }

    fn op_halt(&mut self) {
        self.running = false;
    }

    fn op_push(&mut self) {
        self.ip += 1;
        let value = self.instructions[(self.ip) as usize] as i32;
        self.stack.push(value);
        self.ip += 1;
    }

    fn op_print(&mut self) {
        match self.stack.pop() {
            Some(v) => println!("{}", v),
            None => println!("Error printing value"),
        }
        self.ip += 1;
    }
}
