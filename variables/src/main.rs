use std::io;

fn main() {
    // c++ constexpr
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Constexpr: {THREE_HOURS_IN_SECONDS}");

    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    // shadowing creates new variable
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Spaces: {spaces}");

    let guess: u32 = "42".parse().expect("Not a number");
    println!("Guess {guess}");

    let binary_scalar = 0b1111_0000;
    println!("binary_scalar: {binary_scalar}");

    // IEEE 753
    let double = 2.0;
    let single: f32 = 2.0;

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // Rust char is 4 bytes unicode
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // Compount types

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // c++ structured bindings unpacking o.o
    // Rust calls it destructuring
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    // empty tuple is called unit
    let unit: ();

    // let a = [3, 3, 3, 3, 3];
    let a = [3; 5];

    let first = a[0];
    let second = a[1];

    // data allocated on the stack
    let array = [1, 2, 3, 4, 5];

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let index: usize = input
        .trim()
        .parse()
        .expect("Invalid number");

    unsafe {
        // looks like debug mode will help me
        let unsafe_element = array.get_unchecked(index);
        println!("The value of the unsafe_element at index {index} is: {unsafe_element}");
    }

    let element = array[index];
    println!("The value of the element at index {index} is: {element}");

    another_function();    

}

fn another_function() -> () {
    println!("Another function");
}
