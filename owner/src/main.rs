fn main() {
    let mut s = String::from("hello world!");
    println!("{}", s);
    s = String::from("something else entirely!");
    println!("{}", s);

    // Concept of 'move'.
    // Since 5 is a primitive/literal it will be pushed onto the stack
    // Hence, we know the size of it and attributing y = x will take the copy
    // of x and bind it to y.
    let x = 5;
    let y = x;

    // In the case of a String, a String contains 3 things: a pointer to the memory
    // location of the data, the length, and the capacity. A String is stored on the
    // heap. Since a String may potentially contain a very large amount of data,
    // hard copies are prevented in Rust in order to account for memory safety.
    // In essence, if both variables pointed to the same memory block, due to
    // the nature of how Rust frees variables when their scope exits, it would have
    // freed twice the same pointer, creating a memory corruption error, thus decreasing
    // security. As such, the first variable is invalidated and moved to the second variable.
    let s1 = String::from("Something");
    let s2 = s1;

    // This will not work for the reasons mentioned between 14:21.
    // println!("{}", s1);

    // Some data types have Clone, which states that we can essentially c-style copy them.

    let name = String::from("Michael");
    takes_ownership(name);    // Some function takes ownership of 'name'

    // println!("{} is his name!", name);   // This will not work since ownership is lost.

    let name = String::from("Michael");
    borrows_name(&name);

    let mut name = String::from("Michael");
    borrows_name_writes(&mut name);
    
    println!("{} is the modified name!", name);
}

// Since we take a reference of the variable, the function does not take ownership since it never
// has it. The function is not allowed to modify the variable since we only borrow.
fn borrows_name(name: &String) {
    println!("{} is the length of the name!", name.len());
}

// The function takes full ownership of the variable. 
fn takes_ownership(name: String) {
    println!("{} is the length of the name!", name.len());
}

// The function has write privilages to the variable, but it does not take ownership.
fn borrows_name_writes(name: &mut String) {
    name.push_str(" Bay");
}
