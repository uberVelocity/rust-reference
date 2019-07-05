
// Types can have traits -> T currently has no traits
//pub fn largest<T>(list: &[T]) -> T {
//
//}

// Generics can be used in structs
//pub struct Point<T, U> {
//    x: T,
//    y: U,
//}

//pub fn init_point() {
//    let point1 = Point {x: 3, y: 1.5};
//    let point2 = Point {x: 1, y: 3};
//    let point3 = Point {x: 3.3, y: 5.5};
//}

// Generics can be used in Enums
//enum Option(T) {
//    Some(T),
//    None,
//}

// Enums can hold multiple generics
//enum Result<T, E> {
//    Ok(T),
//    Err(E),
//}

struct Point3D<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Point3D<T> {
    pub fn x(&self) -> &T {
        &self.x
    }
}

pub fn test_3D() {
    let p = Point3D {x: 3, y:4, z:5};

    println!("p.x = {}", p.x());
}