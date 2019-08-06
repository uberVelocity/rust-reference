// pub fn run() {
//     let r;

//     {
//         let x = 5;
//         r = &x;
//     }
//     // 'x' scope ends at the block, so 'r' is a dangling reference

//     println!("r: {}", r);
// }

// We don't know which borrowed reference will be returned (x or y).
// So the output parameter requires a generic lifetime parameter. 
// pub fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } 
//     else {
//         y
//     }
// }

// Specifying lifetime parameters:
// &i32             reference to an i32
// &'a i32          reference to an i32 with an explicit lifetime
// &'a mut i32      mutable reference to an i32 with an explicit lifetime 

// If two variables have the lifetime 'a, it means that both of them must live 
// as long as the generic lifetime 'a.

// Lifetime annotation restricts the lifetime of the parameters to be equal to
// the smallest lifetime of x and y.
pub fn longest2<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}

pub fn run2() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest2(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// Lifetime restricton example
pub fn lifetime_restriction() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest2(string1.as_str(), string2.as_str());
        println!("The longest string isi {}", result);
    }
    // Both references end here, so the borrow checker approves! Longest string is long :3

    // Example that does not work since references have different lifetimes
    let result;
    {
        let string2 = String::from("xyz");
        result = longest2(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);

}