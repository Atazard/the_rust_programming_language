fn main() {
    let n = 5;

    let n_to_fahrenheit = convert_celsius_to_fahrenheit(n);
    println!("{n} celsius is {n_to_fahrenheit} fahrenheit");
    println!();

    let n_to_celsius = convert_fahrenheit_to_celsius(n);
    println!("{n} fahrenheit is {n_to_celsius} celsius");
    println!();

    let nth_fib = get_nth_fibonacci(n);
    println!("Fibonacci {n}: {nth_fib}");
    println!();

    sing_christmas_carol();
}

fn sing_christmas_carol() {
    const ARR: [&str; 12] = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for i in 1..13 {
        println!("On day {i} of Christmas, my true love sent to me");
        for j in (1..i + 1).rev() {
            println!("{}", ARR[j - 1]);
        }
        println!();
    }
}

fn get_nth_fibonacci(n: i32) -> i32 {
    if n < 0 {
        panic!("{n} is negative!");
    }

    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut sum = 0;
            let mut last = 0;
            let mut curr = 1;

            for _i in 1..n {
                sum = last + curr;
                last = curr;
                curr = sum;
            }
            sum
        }
    }
}

#[test]
fn test_get_nth_fibonacci() {
    assert_eq!(get_nth_fibonacci(0), 0);
    assert_eq!(get_nth_fibonacci(1), 1);
    assert_eq!(get_nth_fibonacci(2), 1);
    assert_eq!(get_nth_fibonacci(3), 2);
    assert_eq!(get_nth_fibonacci(4), 3);
    assert_eq!(get_nth_fibonacci(5), 5);
    assert_eq!(get_nth_fibonacci(6), 8);
    assert_eq!(get_nth_fibonacci(7), 13);
    assert_eq!(get_nth_fibonacci(8), 21);
    assert_eq!(get_nth_fibonacci(9), 34);
    assert_eq!(get_nth_fibonacci(10), 55);
}

fn convert_fahrenheit_to_celsius(temperature: i32) -> i32 {
    (temperature - 32) * 5 / 9
}

#[test]
fn test_convert_fahrenheit_to_celsius() {
    assert_eq!(convert_fahrenheit_to_celsius(32), 0);
    assert_eq!(convert_fahrenheit_to_celsius(68), 20);
    assert_eq!(convert_fahrenheit_to_celsius(104), 40);
}

fn convert_celsius_to_fahrenheit(temperature: i32) -> i32 {
    (temperature * 9 / 5) + 32
}

#[test]
fn test_convert_celsius_to_fahrenheit() {
    assert_eq!(convert_celsius_to_fahrenheit(0), 32);
    assert_eq!(convert_celsius_to_fahrenheit(20), 68);
    assert_eq!(convert_celsius_to_fahrenheit(40), 104);
}
