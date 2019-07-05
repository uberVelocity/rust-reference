use std::collections::HashMap;
mod test_hash;
// Hash map stores values on the heap

fn main() {
    test_hash::bdo();
    let mut scores = HashMap::new();
    let favorite_color = String::from("Yellow");

    // Insert values into HashMap. (String, i32) <-> (key, value).
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Accept touple by zipping and collecting values from vectors.
    let v = vec![String::from("Blue"), String::from("Yellow")];
    let i_s = vec![10, 50];

    let mut scores: HashMap<_,_> = v.iter().zip(i_s.iter()).collect();

    // Types that implement copy trait do not give ownership to hashmap.
    // The same cannot be said about types that do not implement copy. The
    // HashMap is the new owner. 
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // Ownership is lost to hashmap -> field_value and field_name cannot be used
    // from here.

    // Retrieval of item from hashmap is done with get(). Provide key to hashmap.
    let f_color = scores.get(&String::from("Yellow"));

    // Iterate over keys and values in hashmap:
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Writting a value at the same key will overrwide the old value with the new one.
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // We could also only insert a value if the key has no value.
    // This is done using the Entry API
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // Updating a value based on the old value
    let text = "normal cars are jealous of flying cars";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
