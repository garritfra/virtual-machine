use stack::Stack;

#[derive(Debug)]
pub struct VM {
  ip: u8,
  instructions: Vec<u8>,
  stack: Stack,
  running: bool,
}

impl VM {
  pub fn new() -> VM {
    VM {
      ip: 0,
      instructions: vec![],
      stack: Stack::new(1024),
      running: false,
    }
  }

  pub fn load_instructions(&mut self, instructions: Vec<u8>) {
    self.instructions = instructions;
  }

  pub fn run(&mut self) {
    self.running = true;
    while self.running {
      match self.instructions[(self.ip) as usize] {
        0x00 => self.op_nop(),
        0x01 => self.op_push(),
        0x02 => self.op_print(),
        0x03 => self.op_add(),
        0xff => self.op_halt(),
        _ => self.op_halt(),
      }
    }
  }

  pub fn op_nop(&mut self) {
    self.ip += 1;
  }

  pub fn op_halt(&mut self) {
    self.running = false;
  }

  pub fn op_push(&mut self) {
    self.ip += 1;
    let value = self.instructions[(self.ip) as usize] as i32;
    self.stack.push(value);
    self.ip += 1;
  }

  pub fn op_print(&mut self) {
    match self.stack.pop() {
      Some(v) => println!("{}", v),
      None => println!("No value to print!"),
    }
    self.ip += 1;
  }

  pub fn op_add(&mut self) {
    if let Some(v1) = self.stack.pop() {
      if let Some(v2) = self.stack.pop() {
        self.stack.push(v1 + v2);
      }
    }
    self.ip += 1;
  }
}
