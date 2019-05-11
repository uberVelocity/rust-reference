fn main() {
    let number = 3;
    let condition = true;

    if number < 3 {
        println!("Condition was true!");
    }
    else {
        println!("Condition was false!");
    }
    
    let number = if condition {
        5
    }
    else {
        6
    };

    println!("number = {}", number);

    /**
     * Rust needs to know at compile time the type 
     * of 'number' through 'let'. Since it expects
     * a u32 but else returns str. So Rust panics.
    */
    /*let number = if condition {
        5
    }
    else {
        "six"
    };*/

    println!("number = {}", number);

    let mut counter: u32 = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2
        }
    };

    println!("result = {}", result);

    let number = 3;
    counter = 0;

    while counter != number {
        println!("counter at {}!", counter);

        counter += 1;
    }

    println!("Out of loop!");


    let array = [10, 20, 30, 40, 50];
    let mut index = 0;

    // Less safe since index may be wrong.
    // Slow because compiler adds runtime code to perform
    // the conditional check every element on every iteration
    // through the loop.
    while index < 5 {
        println!("the value is: {}", array[index]);

        index = index + 1;
    }

    for element in array.iter() {
        println!("The value is: {}", element);
    }

    for countdown in (1..4).rev() {
        println!("{}", countdown);
    }
    println!("LIFTOFF!!!");

    /*
    if number { // This shit doesn't fly here.
        println!("Number is different than zero");
    }*/
}
