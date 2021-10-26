use stack::Stack;

#[derive(Debug)]
/// Virtual Machine struct
pub struct VM {
    ip: u8,
    instructions: Vec<u8>,
    stack: Stack,
    registers: [u8; 16],
    memory: [u8; 1024],
    running: bool,
}

impl VM {
    /// Virtual Machine Constructor.
    /// Initializes Instruction ponter to 0 and the stack to a size 1024 `Stack` object
    pub fn new() -> VM {
        VM {
            ip: 0,
            instructions: vec![],
            stack: Stack::new(1024),
            memory: [0x0; 1024],
            registers: [0x0; 16],
            running: false,
        }
    }

    /// Load a program into memory. Program is provided as `Vec<u8>`
    pub fn load_instructions(&mut self, instructions: Vec<u8>) {
        self.instructions = instructions;
    }

    /// Run Program.
    /// Sets the `running` property to true and starts iterating over the instructions
    pub fn run(&mut self) {
        self.running = true;
        while self.running {
            match self.instructions[(self.ip) as usize] {
                // TODO: These are not up to date anymore
                0x00 => self.op_nop(),
                0x01 => self.op_push(),
                0x02 => self.op_print(),
                0x03 => self.op_add(),
                0xff => self.op_halt(),
                _ => self.op_halt(),
            }
        }
    }

    /// No Operation.
    pub fn op_nop(&mut self) {
        self.ip += 1;
    }

    /// Stop the program
    pub fn op_halt(&mut self) {
        self.running = false;
    }

    /// Push the next value onto the stack
    pub fn op_push(&mut self) {
        self.ip += 1;
        let value = self.instructions[(self.ip) as usize] as i32;
        self.stack.push(value);
        self.ip += 1;
    }

    /// Pop a value of the stack and print it
    pub fn op_print(&mut self) {
        match self.stack.pop() {
            Some(v) => println!("{}", v),
            None => println!("No value to print!"),
        }
        self.ip += 1;
    }

    /// Pop two values of the stack, add them and push the result on the stack
    pub fn op_add(&mut self) {
        if let Some(v1) = self.stack.pop() {
            if let Some(v2) = self.stack.pop() {
                self.stack.push(v1 + v2);
            }
        }
        self.ip += 1;
    }
}
