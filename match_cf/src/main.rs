#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Penny;
    println!("The value of the coin is: {}\n", value_in_cents(coin));

    let coin = Coin::Quarter(UsState::Alabama);
    println!("The value of the coin is: {}\n", value_in_cents(coin));

    let five = Some(5);
    let _six = add_one(five);
    let _none = add_one(None);

    let roll = 9;
    match roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // like _, use it if you plan to use the value
        // _ => reroll() // catch-all but you don't need the value
        // _ => () // catch-all, do nothing
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn add_one(x: Option<i32>) -> Option<i32> {
    // manual impl of Option::map?
    // x.map(|i| i + 1)
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}

fn remove_fancy_hat() {}

fn move_player(value: u8) {}
