fn main() {
    let string = String::from("This is some sick ass string");

    let slice = &string[0..3];
    println!("Slice: {}", slice);

    let end = &string[7..];
    println!("End: {}", end);

    let start = &string[..4];
    println!("Start = {}", start);

    let whole = &string[..];
    println!("Whole = {}", whole);

    let f_three = first_three(&string);

    println!("f_three = {}", f_three);

    let new_var = get_variable(&string);

    println!("new_var = {}", new_var);
}

fn first_three(name: &str) -> &str {
    &name[0..3]
}