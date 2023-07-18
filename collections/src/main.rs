use std::collections::HashMap;

fn main() {
    create_update_read_vector();

    iterate_over_vector_values();

    enum_in_vector();

    fun_with_strings();

    fun_with_hash_maps();
}

fn fun_with_hash_maps() {
    let mut scores = HashMap::new();
    scores.insert("Red", 69);
    scores.insert("Blue", 420);

    let team_name = "Red";

    println!("{} teams score: {}", team_name, scores.get(team_name).copied().unwrap_or(0));
    println!();

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    println!();

    let v = vec!["Red", "Green", "blue"];

    for key in v {
        println!("{}: {}", key, scores.get(key).copied().unwrap_or(0));
    }
    println!();

    println!("scores: {:#?}", scores);
    println!();

    let key = "Blue";
    scores.insert(key, 421); // replaces old value
    println!("scores: {:#?}", scores);
    println!();

    scores.entry("Green").or_insert(1337); // this should add green with 1337
    let entry = scores.entry("Red").or_insert(70); // this should not change red
    println!("{entry}"); // .entry returns &mut with the value at or_insert()

    println!("scores: {:#?}", scores);
    println!();

    let text = "hello world hello me"; // NLP potential Python is jealous
    let mut word_count_map = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_count_map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("word count map: {:#?}", word_count_map);
    println!();
}

fn fun_with_strings() {
    let mut s = "foo".to_string();
    s.push(' ');
    s.push_str("bar");
    println!("s: {}", s); // foo bar

    let last = s.pop();
    match last {
        Some(x) => println!("{x}"), // r
        None => println!("Nope")
    }

    let s = "foo".to_string() + " bar"; // no need for "bar".to_string()
    println!("{s}"); // foo bar

    let s = format!("{} {}", "test", "tost");
    println!("{s}"); // test tost

    let slice = &s[..4];
    println!("{}", slice); // test

    println!();
}

fn enum_in_vector() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        SpreadsheetCell::Text(String::from("value")),
        SpreadsheetCell::Float(6.9),
        SpreadsheetCell::Int(420)
    ];

    for element in row {
        println!("element: {:?}", element);
    }

    println!();
}

fn iterate_over_vector_values() {
    let v = vec![69, 420, 1337];
    for i in v {
        println!("{i}"); // i is i32
    }
    println!();

    let v = vec![69, 420, 1337];
    for i in &v {
        println!("{i}"); // i is &i32
    }
    println!();

    let v = vec![69, 420, 1337];
    for i in &v {
        println!("{}", *i); // i is &i32
    }
    println!();

    let mut v = vec![69, 420, 1337];
    for i in &mut v {
        *i += 1;
        println!("{i}"); // i is &mut i32 and you need to dereference it to mutate it
    }

    println!();
}

fn create_update_read_vector() {
    let mut v: Vec<i32> = Vec::new();

    let v2 = vec![1, 2, 3];

    v.push(4);
    v.push(5);
    v.push(6);

    let value = &v[2];
    println!("value: {}", value); // 6

    let value = v2.get(2); // 3
    match value {
        Some(value) => println!("value: {}", value),
        None => println!("No element"),
    }

    let value = v2.get(3);  // out of bounds
    match value {
        Some(value) => println!("value: {}", value),
        None => println!("No element"),
    }

    let mut v = vec![1, 2, 3];

    let first = &v[0];
    println!("first element: {}", first); // 1

    v.push(4);

    // println!("first element: {}", first); // can't print here

    println!();
}
