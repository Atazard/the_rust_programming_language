use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x outside is: {x}");

    let spaces = "    ";
    let spaces = spaces.len();
    println!("There are {spaces} spaces");

    println!("sum: {}", 5 + 10);
    println!("diff: {}", 95.5 - 4.3);
    println!("prod: {}", 4 * 30);
    println!("quotient: {}", 56.7 / 32.2);
    println!("truncated: {}", -5 / 3);
    println!("rem: {}", 43 % 5);

    let _cat = '😻';

    let tup = (2.0, -3, true);
    let (x, y, z) = tup;
    println!("{} {} {}", x, y, z);
    println!("{}", tup.2);

    const MONTHS: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let array = [1; 5];
    println!("{} {}", array[0], MONTHS[6]);

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index : usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    println!("The value of the element at index {index} is: {}", MONTHS[index]);
}
