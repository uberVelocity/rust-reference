fn another_function(age: u32, height: f32) {
    println!("Name:{} \n Age:{}", height, age);
}

// Implicity returns the last expression.
fn five() -> u32 {
    5
}

// Return early using 'return'.
fn seven() -> u32 {
    return 7;
}

fn plus_one(number: u32) -> u32 {
    number + 1
}

fn funky_block() {
    let x = 5;

    // Expression inside the block.
    let y = {
        let x = 3;
        x + 1   // Notice the absence of ;
    };

    println!("The value of y is: {}", y);
}

fn main() {
    println!("Hello, world!");

    another_function(22, 182.5);

    funky_block();

    let five: u32 = five();
    let seven: u32 = seven();
    
    println!("five = {} \nseven = {}", five, seven);

    println!("eight = {}", plus_one(seven));

}
