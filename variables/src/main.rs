fn main() {
    const MAX_POINTS: u32 = 100_000;

    let x = 5;

    let x = x + 1;

    let x = x * 2;

    let fl = 2.0;   // f64

    let fl: f32 = 3.0;  // f32

    let isSleepy: bool = false;

    let c = 'z';

    // Touples
    let person = ("Jim", false, 30);
    let (u, v, w) = person;
    
    let name = person.0;
    let married = person.1;
    let age = person.2;
    
    println!("The value of v = {}", v);

    let guess: u32 = "42".parse().expect("Not a number!");
    
    let array = [1, 2, 3, 4, 5];

    let otherArray: [u32; 3] = [1, 2, 3];

    // Contains 30 values of 10.
    let sameValueNTimes = [10; 30];
    
    let b = sameValueNTimes[29];

    /**
     *  If overflow occurs when compiling in release mode
     * Rust performs two's complement wrapping. Values greater
     * than the maximum value of the type can hold 
     * "wrap around" to the minimum of the values the type can hold.
     * In the case of u8, 256 becomes 0, 257 becomes 1 etc.
     * 
     * Relying on wrapping behavior is considered an error.
     */
    let integerOverflow: u8 = 255;
    println!("The value of x: {}", x);
    println!("The value of MAX_POINTS: {}", MAX_POINTS);
    let spaces = "    ";
    let spaces = spaces.len();
}
