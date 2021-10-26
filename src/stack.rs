#[derive(Debug)]
/// Implementation of a virtual stack.
pub struct Stack {
    top: u32,
    size: u32,
    stack: Vec<i32>,
}

/// Provides methods to perform operations on the stack
impl Stack {
    /// Create an empty stack with the given size
    pub fn new(size: u32) -> Stack {
        Stack {
            top: 0,
            size: size,
            stack: Vec::new(),
        }
    }

    /// Push a new `i32` value to the stack
    pub fn push(&mut self, value: i32) {
        self.stack.push(value);
        self.top += 1;
    }

    /// Pop a value from the stack. Returns the value as `i32`, if the stack is not empty
    pub fn pop(&mut self) -> Option<i32> {
        self.top -= 1;
        self.stack.pop()
    }
}
