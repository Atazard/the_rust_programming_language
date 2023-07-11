fn main() {
    example1();
    example2();
    example3();
    example4();
    example5();
}

fn example5() {
    println!("Example 5:"); // Much better

    for number in (1..4).rev() { // why does this not go to 4?
        println!("{number}");
    }
    println!("Liftoff!");

    println!();
}

fn example4() {
    println!("Example 4:");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    println!()
}

fn example3() {
    println!("Example 3:"); // This is not the best way to do something like this

    let mut count = 3;

    while count != 0 {
        println!("{count}");
        count -= 1;
    }

    println!("Liftoff!");

    println!();
}

fn example2() {
    println!("Example 2:");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    println!();
}

fn example1() {
    println!("Example 1:");

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    println!();
}
