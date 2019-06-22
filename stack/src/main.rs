mod stack;
mod iohandle;

macro_rules! show {
    ($stack:expr) => {
        stack::Stack::show_stack($stack);
    };
}


fn main() {
    iohandle::init();
    let stack_size = iohandle::input_stack_size();
    let mut new_stack = stack::Stack::new_stack(stack_size);    // I think stack_size loses scope here. It does not.
    
    show!(&new_stack);
    new_stack.push(10);
    show!(&new_stack);
    new_stack.push(42);
    show!(&new_stack);
}
