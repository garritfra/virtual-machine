#[derive(Debug)]
pub struct Stack {
  top: u32,
  size: u32,
  stack: Vec<i32>,
}
impl Stack {
  pub fn new(size: u32) -> Stack {
    Stack {
      top: 0,
      size: size,
      stack: Vec::new(),
    }
  }

  pub fn push(&mut self, value: i32) {
    self.stack.push(value);
    self.top += 1;
  }

  pub fn pop(&mut self) -> Option<i32> {
    self.top -= 1;
    self.stack.pop()
  }
}
