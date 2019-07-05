mod err_types;

fn main() {
    let s = err_types::read_username_from_file();
    println!("s: {:?}", s);

    // Outputs only differ because different text files are used
    let s = err_types::short_read_username_from_file();
    println!("s: {:?}", s);
}
