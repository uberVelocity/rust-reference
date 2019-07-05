use std::fs;
use std::io;
use std::fs::File;
use std::io::ErrorKind;
use std::io::Write;
use std::io::Read;

// Two types of errors: Recoverable or Unrecoverable.

/**
 * Panicking by default initiates unwinding. If, for instance, abort is preferred,
 * change Cargo.toml [profile] section by adding:
 * [profile.release]
 * panic = 'abort'
 */
pub fn unrecoverable_err() {
    // Invoking unrecoverable error with panic! macro
    //panic!("Yikes! That did not go as planned now did it Michael?");

    let v = vec![1, 2, 3];

    // Buffer overread. In C, this returns something, namely the value at that address in memory.
    // This is susceptible to attackers that read data found at that addresses. Rust does not allow this
    v[99];
}

/** T and E are generics
 * enum Result<T, E> {
 *  Ok(T),
 *  Err(E),
 * }
 * 
 * A result can be either OK or Err
 * T = type of value returned in the success case
 * E = type of error returned in the failure case
 */

pub fn recoverable_err() {
    // If we ever want to find out the return type of a method, we can assign a type to the variable
    // that stores the method's result to something that we know it cannot return. The compiler tells you
    // what type it returns. If we say
    
    // let f: u32 = File::open("file1.txt");

    // Compiler will output: found type `std::result::Result<std::fs::File, std::io::Error>
    //                                    return type ^        T type ^       E type ^
    // So, if it succeeds, it returns a Result instance of Ok(T) with type of T = File

    // Attempt to open file `file2.txt`
    let f = File::open("file1.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("file1.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        }
        else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

pub fn unwrap_test() {
    let f = File::open("file1.txt").unwrap();  // If Ok(T) variant is returned, unwrap() returns the value T
    
    // expect() is equivalent to unrwap(), but allows
    // for a message to be passed in for debugging
    let f = File::open("file1.txt").expect("Unable to open the file!");

    // This envokes the panic! macro with the argument of expect() as an argument to display on screen
    let f = File::open("file2.txt").expect("Unable to open the file!");
}

pub fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    
    let mut f = File::open("file1.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// Same functionality as previous function, but shorter
pub fn short_read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("file2.txt")
}