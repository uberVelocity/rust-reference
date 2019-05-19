enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

/**
 * The following structs hold the same
 * infomration as the Enum 'Message' above.
 */
struct QuitMessage;  // Unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);  // Tuple struct
struct ChangeColorMessage(i32, i32, i32);  // Tuple struct


fn route(ip_type: IpAddrKind) {

}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => {
            println!("oooo a quarter!");
            25
        }
    }
}

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);

    let loopback = IpAddrKind::V6(String::from("::1"));

    let mut m = Message::Write(String::from("hello"));

    let m2 = Message::Move{x:30, y:10};

    let m3 = Message::Quit;
    
    let m4 = Message::ChangeColor(10, 20, 30);

    // Change value of enum type by calling
    // its type as in the definition.
    m = Message::Write(String::from("something"));

    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);  // Value can be empty with Option.
    assert_eq!(y.is_some(), true);

    // let sum = x + y; // Conversion from Option<T> to T required
    // before being able to sum.

    // Not having to worry about incorrectly assuming a not-null value
    // helps you to be more confident in your code. In order to have a 
    // value that can possibly be null, you must explicitly opt in by 
    // making the type of that value Option<T>. Then, when you use that 
    // value, you are required to explicitly handle the case when the value 
    // is null. Everywhere that a value has a type that isn’t an Option<T>, you 
    // can safely assume that the value isn’t null. This was a deliberate design 
    //decision for Rust to limit null’s pervasiveness and increase the safety of Rust 
    //code.

    // -------- MATCH --------
    let nick = Coin::Nickel;
    println!("A nickel is {} cents!", value_in_cents(nick));
    let quarter = Coin::Quarter;
    println!("{}", value_in_cents(quarter));
}
