fn main() {
    // loop_labels();
    // lift_off();
    // lift_off_v_for();
    // for_loop_example();
    let fib = nth_fibonacci_number(10);
    let fib2 = nth_fibonacci_number_recur(10);
    println!("fib: {fib}");
    println!("fib2: {fib2}");
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

fn lift_off() {
    let mut number = 3;
    while number >= 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn lift_off_v_for() {
    for number in (1..4).rev() {
        println!("{number}!");
    }

    println!("LIFTOFF!!!");
}

fn for_loop_example() {
    let a = [10, 20, 30, 40, 50];
    
    // compiler can optimize the index check when doing this
    // when accessing element by index Rust compiler by default adds runtime check bounds
    // a[999] will throw runtime error
    for element in a {
        println!("the value is: {element}");
    }
}

fn nth_fibonacci_number(n: u32) -> u32 {
    if n == 1 || n == 2 {
        return 1;
    }

    let mut curr = 1;
    let mut last = 1;

    for _ in 0..(n-2) {
        let old_curr = curr;
        curr += last;
        last = old_curr;
        
    }

    return curr;
}

fn nth_fibonacci_number_recur(n: u32) -> u32 {
    if n == 1 || n == 2 {
        return 1;
    }

    return nth_fibonacci_number_recur(n-1) + nth_fibonacci_number_recur(n-2);
}
