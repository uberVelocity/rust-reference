ls
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated function that, in this case, is used as a 
    // constructor. To call it, use ::
    // let rect = Rectangle::square(3);
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size}
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

struct User {
    name: String,
    age: u32,
    male: bool,
}

fn build_user(name: String, age: u32, male: bool) -> User {
    User {
        name,
        age,
        male
    }
}

fn print_user_info(user: &User) {
    println!("User name: {}", &user.name);
    println!("User age: {}", &user.age);
    println!("User male?: {}", &user.male);
}

fn print_color(color: &Color) {
    println!("Color is ({}, {}, {})", &color.0, &color.1, &color.2);    // Ampersands are optional
}

fn print_point(point: &Point) {
    println!("Point is ({}, {}, {})", &point.0, &point.1, &point.2);    // Ampersands are optional
}

fn main() {
    let user1 = User {
        name: String::from("Mihai Popescu"),
        age: 22,
        male: true,
    };

    let user2 = User {
        name: String::from("Alex Tutea"),
        ..user1
    };

    let user3 = build_user(String::from("Radu Catarambol"), 20, true);

    print_user_info(&user1);
    print_user_info(&user2);
    print_user_info(&user3);

    let origin = Point(0, 0, 0);
    let black = Color(0, 0, 0);

    print_color(&black);
    print_point(&origin);

    let rect = Rectangle { width: 30, height: 50};
    let other_rect = Rectangle { width: 10,  height: 10};

    println!("Area of rect = {}", rect.area());

    println!("rect is {:#?}", rect);

    println!("Can rect hold other_rect? {}", rect.can_hold(&other_rect));

    let associated_rect = Rectangle::square(3);
    println!("associated rectangle = {:?}", associated_rect);
}