/* Stack implementation able to store integers in RUST. The stack has a dynamic size. */

#[derive(Debug)]    // Debug modifier so we can inspect it.
pub struct Stack {
    size: u32,
    stack: Vec<u32>,    // Usage of `Vec` since we want the stack to be dynamic.
}

impl Stack {
    // Creates new stack of size `size` and returns it.
    pub fn new_stack(size: u32) -> Stack {
        Stack {
            size: size,
            stack: Vec::new(),
        }
    }

    // Prints the stack using debug derivation.
    pub fn show_stack(stack: &Stack) {
        println!("prinited stack:{:?}", stack);
    }

    // Checks if stack is empty
    pub fn is_empty(&self) -> bool {
        match self.stack.len() {
            0 => {
                println!("Stack is empty...");
                return true;
            }
            _ => false
        }
    }

    // Pops an element from the stack
    pub fn pop(&mut self) -> u32 {
        match self.stack.pop() {
            Some(x) => {
                println!("Popping: {}...", x);
                return x;
            },
            None => {
                println!("Invalid pop - vec could be empty...");
                return 0;
            }
        }
    }

    // Pushes an element on the stack
    pub fn push(&mut self, number: u32) {
        self.stack.push(number);
    }

}

