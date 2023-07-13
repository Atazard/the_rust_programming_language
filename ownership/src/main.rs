fn main() {
    let s1 = gives_ownership();
    println!("{s1}");

    let s2 = String::from("test");

    let s3 = takes_and_gives_back(s2);
    println!("{s3}");

    //this is not the way
    //pass by reference instead (borrowing)
    let (s3, len) = calculate_length(s3);
    println!("The length of '{s3}' is {len}");

    //this is the way
    let s4 = String::from("hello");
    let len = calculate_length2(&s4);
    println!("The length of '{s4}' is {len}");

    //why use a mut ref?
    let mut s = String::from("test");
    change(&mut s);
    println!("{s}");

    //multiple immutable refs are ok
    let r1 = &s;
    let r2 = &s;
    println!("{r1} and {r2}");

    //only 1 immutable ref is ok
    let r3 = &mut s;
    println!("{r3}");
    //you can't print the immutable refs here
    // println!("{r1}, {r2} and {r3}"); //this is a no no

    let s = String::from("hello world");
    let slice1 = &s[..5];
    let slice2 = &s[6..];
    println!("{slice1}, {slice2}!");

    let slice = first_word(&s);
    println!("{slice}");
    let slice = second_word(&s);
    println!("{slice}");

    let slice = first_word(&s[..5]);
    println!("{slice}");
    let slice = first_word("test");
    println!("{slice}");
}

fn gives_ownership() -> String {
    String::from("temp")
}

fn takes_and_gives_back(value: String) -> String {
    value
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

//sadly no function overloading :(
//maybe traits???
fn calculate_length2(s: &String) -> usize {
    s.len()
}

fn change(string: &mut String) {
    string.push_str("ing");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    s
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[(i+1)..];
        }
    }
    "" //what would you return if you didn't find a second word?
}
