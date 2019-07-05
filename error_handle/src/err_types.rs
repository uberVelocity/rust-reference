use std::fs::File;
use std::io::Write;

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
    let f = File::open("file1.txt");

    // Match the value of f and assign that to f
    let f = match f {
        Ok(file) => file,     // If f was opened, return the handle to that file
        Err(error) => match error.kind() {      // If f was not opened, an error was returned -> match the error kind
            // ErrorKind::NotFound is a specific kind of error. If it ocurred, attempt to create
            // the file on the disk.
            ErrorKind::NotFound => match File::create("file2.txt") {
                Ok(fc) => fc,      // If we successfuly created the file, return it.
                Err(e) => {        // Else, panic!
                    panic!("Problem creating the file: {:?}", e),
                },
            },
            // If the open was not possible due to some other error, panic!
            other_error => panic!("Problem opening the file: {:?}", other_error);
        },
    };
}