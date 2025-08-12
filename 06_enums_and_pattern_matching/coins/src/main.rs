#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    // snip...
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_state) => 25,
    }
}

fn main() {
    let mut count = 0;

    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter = Coin::Quarter(UsState::Alabama);

    for (i, coin) in [penny, nickel, dime, quarter].iter().enumerate() {
        println!(
            "Coin {} is {:?} and value is {}",
            i,
            coin,
            value_in_cents(coin)
        );
        match coin {
            Coin::Quarter(state) => println!("State quarter from {:?}!", state),
            _ => count += 1,
        }
    }
    println!("Count is {}", count);
}
