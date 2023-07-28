#![allow(unused)]
use std::{
    error::Error,
    fs::{self, File},
    io::{Error as OtherError, Read},
};

fn main() -> Result<(), Box<dyn Error>> {
    // match read_username_from_file() {
    //     Ok(username) => println!("username: {}", username),
    //     Err(error) => panic!("oopsie!! {}", error),
    // }

    // let greeting_file = File::open("hello.txt")?;

    assert_eq!(get_last_char_of_first_line("Hello world!\ntest"), Some('!'));
    assert_eq!(get_last_char_of_first_line(""), None);
    assert_eq!(get_last_char_of_first_line("\ntest"), None);

    Ok(())
}

// You would probably open the file in another function
// and have this function get a file and only do the reading part
fn read_username_from_file() -> Result<String, OtherError> {
    // This is not the way
    // let username_file_result = File::open("hello.txt");

    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };

    // let mut username = String::new();

    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }

    // Much better
    // let mut username_file = File::open("hello.txt")?;
    // let mut username = String::new();
    // username_file.read_to_string(&mut username)?;
    // Ok(username)

    // Even better-er
    // let mut username = String::new();

    // File::open("hello.txt")?.read_to_string(&mut username)?;

    // Ok(username)

    // If we assume the file only contains the username we can go even further beyond!
    fs::read_to_string("hello.txt")
}

fn get_last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
