use std::collections::HashMap;

fn main() {
    
    // stores a mapping of key of type K
    // to values of type V.
    // hashmaps are stored on the heap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let _score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // for owned values like String, the values will be moved
    // and the hash map takes ownership.
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value); // field_name and field_value are invalid at this point
    
    // overwriting value associated to a key
    scores.insert(String::from("Red"), 10);
    scores.insert(String::from("Red"), 25);

    // adding a key and value only when the key isn't present
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50); // works
    scores.entry(String::from("Blue")).or_insert(50);   // won't work (blue still 10)
    
    println!("{:?}", scores);

    // update value based on the old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

}
