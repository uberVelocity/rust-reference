
#[derive(Debug)]
pub struct Dfsm {
    states: Vec<u8>,
    final_states: Vec<u8>,
    start_state: u8,
    current_state: u8,
    alphabet: Vec<char>,
}

impl Dfsm {
    // Creates a DFSM.
    pub fn init_dfsm() -> Dfsm {
        Dfsm {
            states: Vec::new(),
            final_states: Vec::new(),
            start_state: 0,
            current_state: 0,
            alphabet: Vec::new(),
        }
    }

    // Prints the state of the DFSM.
    pub fn show_dfsm(&self) {
        println!("State of DFSM: {:?}", self);
    }

    // Verifies if a symbol is in the alphabet.
    pub fn valid_symbol(symbol: char, dfsm: &Dfsm) -> bool {
        println!("symbol: {}", symbol);
        let mut found: bool = false;
        for i in 0..dfsm.alphabet.len() {
            if symbol == dfsm.alphabet[i] {
                return true;
            }
        }
        return false;
    }

    // Initializes DFSM states from 0 to `n_states`.
    pub fn init_states(&mut self, n_states: u8) {
        for state in 0..n_states {
            self.states.push(state);
        }
    }

    // Initializes DFSM alphabet with `alphabet`.
    pub fn init_alphabet(&mut self, alphabet: &str) {
        for c in alphabet.chars() {
            self.alphabet.push(c);
        }
    }

    pub fn init_f_states(&mut self) {
        self.final_states.push(4);
    }

    pub fn check_symbols(&self, input: &String) -> bool {
        let mut valid: bool;
        for c in input.chars() {
            valid = Dfsm::valid_symbol(c, self);
            if !valid {
                return false;
            }
        }
        return true;
    }

    pub fn terminate() {
        println!("Machine termination.");
        ::std::process::exit(0);
    }

    pub fn run(&mut self, input: &String) {
        println!("In run function");
        for c in input.chars() {
            match self.current_state {
                0 => {
                    match c {
                        'm' => self.current_state = 1,
                        _ => Dfsm::terminate(),
                    }
                },
                1 => {
                    match c {
                        'i' => self.current_state = 2,
                        _ => Dfsm::terminate(),
                    }
                },
                2 => {
                    match c {
                        'h' => self.current_state = 3,
                        _ => Dfsm::terminate(),
                    }
                },
                3 => {
                    match c {
                        'a' => self.current_state = 4,
                        _ => Dfsm::terminate(),
                    }
                },
                4 => {
                    match c {
                        'i' => self.current_state = 4,
                        _ => Dfsm::terminate(),
                    }
                }
                _ => Dfsm::terminate(),
            }
        }
        println!("VALID INPUT");
    }   
}
