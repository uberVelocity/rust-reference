enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
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

}
