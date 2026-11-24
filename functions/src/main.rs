fn main() -> () {
    println!("Hello, world!");

    another_function();
    foo(42);
    print_labeled_measurement(5, 'C');

    // Statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to a resultant value.

    // statement
    let _y = 6;

    /*
        Calling a function is an expression.
        Calling a macro is an expression.
        A new scope block created with curly brackets is an expression
    */
    // interesting rust quirks
    let y = {
        let x = 3;
        x + 1 // note there is no semi colon thats why its expression
    };

    println!("The value of y is: {y}");

    let x = forty_two();
    println!("The value of x is: {x}");

    println!("x plus ons is {}", plus_one(x));

    let number = 7;
    // blocks of code associated with conditions are called "arms"
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condidiont was false");
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    let condition = true;
    let condition_number: i32 = if condition {
        let temp = 1; 
        temp+2
    } else {
        6
    };
    
    println!("condition_number: {condition_number}");

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter >= 10 {
            break counter * 2;
        }
    };
    println!("The result is: {result}");


    loop_labels();

}

fn another_function() -> () {
    println!("Another function");
}

fn foo(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn forty_two() -> i32 {
    42 // no semicolon
}

fn plus_one(x: i32) -> i32 {
    return x+1;
}

fn loop_labels() {
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
}
