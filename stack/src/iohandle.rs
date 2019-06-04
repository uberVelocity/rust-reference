use std::io::{stdin};

pub fn init() {
    println!("Stack builder v1.0");
}

pub fn input_stack_size() -> u32 {
    println!("Stack size: ");
    let mut size = String::new();
    stdin().read_line(&mut size).expect("Could not get user input!");
    let trimmed = size.trim();
    println!("Size: {}", size);
    match trimmed.parse::<u32>() {
        Ok(i) => {
            println!("You have entered {}", i);
            return i;
        },
        Err(..) => {
            println!("Please insert an integer!");
            input_stack_size();
        }
    };
    7878
}