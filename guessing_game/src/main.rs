use std::cmp::Ordering;
use std::io;

use rand::prelude::*;

fn main() {
    let secret_number = rand::rng().random_range(1..=100);
    println!("The secret number is: {secret_number}");

    println!("Guess the number!");

    loop {
        println!("Please input your guess");

        let mut guess = String::new();
        // let mut guess = "rust".to_string();

        let t = io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {guess}");
        println!("T: {t}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let t = match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        };
    }

    // println! shenanegans
    let x = 5;
    let y = 10;
    println!("x = {x} and y + 2 = {}", y + 2);
}
