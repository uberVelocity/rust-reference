use std::collections::HashMap;

pub fn bdo() {
    let mut players: HashMap<_,_> = HashMap::new();

    let names = vec![String::from("Ozan"), String::from("Suciu")];
    let ap = vec![261, 251];

    players = names.iter().zip(ap.iter()).collect();

    println!("Players:\n{:?}", players);
}