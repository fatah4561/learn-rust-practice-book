use std::collections::HashMap;
fn main() {
    // let mut scores: HashMap<String, i32> = HashMap::new(); // type annotated
    let mut scores = HashMap::new(); // type inferred

    scores.insert("Blue".to_string(), 30);
    scores.insert(String::from("Yellow"), 20);

    // get the value by method
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{:?}", score);

    let yellow_score = scores.get(&String::from("Yellow"));
    println!("{:?}", yellow_score);

    // some modifications
    // copied() to get option<i32> not option<&i32>
    // unwrap_or() to get 0 if option<i32> is None
    let blue_score = scores.get(&String::from("Blue")).copied().unwrap_or(0);
    println!("{blue_score}");

    // iterate
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    /*
        for type with copy able trait like i32 the value will be copied
        while type without it like String the value will be moved
     */
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    
    // println!("{}: {}", field_name, field_value);


    // -- replacing value if key is the same
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // -- insert only if key doesn't exists yet
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50); // entry return enum entry 
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores); // blue is still 10

    // -- updating a value based on the old value
    // example counting the words
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1; // we can dereference the count to modify it, because the or_insert return the mutable reference
    }
    println!("{:?}", map);
}
