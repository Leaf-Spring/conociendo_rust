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

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}");
            25
        }
    }
}

fn main() {
    let coin: Coin = Coin::Quarter(UsState::Alaska);

    println!("{}", value_in_cents(coin));

    let coin: Coin = Coin::Quarter(UsState::Alabama);

    println!("{}", value_in_cents(coin));

    let coin: Coin = Coin::Penny;

    println!("{}", value_in_cents(coin));

    let coin: Coin = Coin::Nickel;

    println!("{}", value_in_cents(coin));

    let coin: Coin = Coin::Dime;

    println!("{}", value_in_cents(coin));

    let coin: Coin = Coin::Quarter(UsState::Alabama);
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }
}
