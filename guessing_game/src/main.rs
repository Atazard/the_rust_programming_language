use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
	let random_number = rand::thread_rng().gen_range(69..=420);
	
    // println!("Random number: {random_number}");
    println!("Guess a number between 69 and 420!");
	
    loop {
		let mut guess = String::new();

        println!("Please enter a number between 69 and 420.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&random_number) {
            Ordering::Less => println!("Your guess was smaller than the random number!"),
            Ordering::Greater => println!("Your guess was greater then the random number!"),
			Ordering::Equal => {
                println!("Congrats! You guessed the random number");
                break;
            }
        }
    }
}