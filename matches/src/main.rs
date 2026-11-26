#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    // match keyword followed by an expression
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let alice_dime = Coin::Dime;
    let bob_penny = Coin::Penny;

    let alice_cash = value_in_cents(alice_dime);
    println!("alice has {} amount of money", alice_cash);

    let bob_cash = value_in_cents(bob_penny);
    println!("bob has {} amount of money", bob_cash);

    let my_coin = Coin::Quarter(UsState::Alaska);
    let my_cash = value_in_cents(my_coin);
    println!("I have {} amount of money", my_cash);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
