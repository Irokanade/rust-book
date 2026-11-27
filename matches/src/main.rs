#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // etc
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
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

fn describe_state_quarter_v1(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new"))
        }
    } else {
        None
    }
}

fn describe_state_quarter_v2(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new"))
    }
}

fn describe_state_quarter_v3(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        println!("coin is None!");

        // rust let-else syntax requires that the else block must diverge
        // eg return, break, continue, panic, etc
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new"))
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

    if let Some(six) = six {
        println!("{}", six);
    }

    // rust match must be exhaustive
    match none {
        Some(none) => {
            println!("none var is Some({})", none);
        }
        None => {
            println!("none var is none");
        }
    }

    // catch-all patterns and the _ Placeholder
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
        // _ => (), // do ntg and return unit type (emtpy tuple type)
    }

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    let mut count = 0;
    let coin = Coin::Dime;
    match coin {
        // match will move my varaible
        // coin is not copiable so borrow instead
        // coin will bind to state aka move
        Coin::Quarter(ref state) => println!("State quarter from {state:?}!"),
        _ => count += 1,
    }

    println!("count is now {count}");

    if let Coin::Quarter(ref state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }

    println!("count is now {count}");
}

fn add_fancy_hat() {
    println!("add fancy hat!");
}

fn remove_fancy_hat() {
    println!("remove fancy hat");
}

fn move_player(num_spaces: u8) {
    println!("move {} amount of spaces", num_spaces);
}
