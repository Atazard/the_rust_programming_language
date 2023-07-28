#![allow(unused)]
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let config_max = Some(3u8); // what is 3u8?

    // This is not the way
    // match config_max {
    //     Some(max) => println!("config max is: {}", max),
    //     _ => ()
    // }

    if let Some(max) = config_max {
        println!("config max is: {}\n", max)
    }

    let mut count = 0;

    let coin = Coin::Quarter(UsState::Alabama);
    count = check_coin(&coin, count);
    println!("Total non-quarters found: {}\n", count);

    let coin = Coin::Penny;
    count = check_coin(&coin, count);
    println!("Total non-quarters found: {}\n", count);
}

fn check_coin(coin: &Coin, mut count: i32) -> i32 {
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state)
    } else {
        count += 1;
        println!("Non-quarter found.")
    }
    count
}
