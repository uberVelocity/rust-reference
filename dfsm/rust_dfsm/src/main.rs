use std::env;
mod dfsm;

fn main() {
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let args: Vec<String> = env::args().collect();
    let mut dfsm = dfsm::Dfsm::init_dfsm();
    dfsm.init_states(5);
    dfsm.init_alphabet(alphabet);
    dfsm.init_f_states();
    dfsm.show_dfsm();

    
    if args.len() > 1 {
        println!("Args 1: {}", args[1]);
        if dfsm.check_symbols(&args[1]) {
            dfsm.run(&args[1]);
        }
    }
    else {
        println!("Empty argument... aborting :(");
    }
}