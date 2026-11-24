fn main() {
    let mut s1 = String::from("hello");
    s1.push_str(", world!");

    // shallow copy
    // s1 move into s2 therefore s1 is invalid after being moved
    // if just plain shallow copy both s1 and s2 would try to free the same string block double free
    // Rust will never automatically deep copy
    let s2 = s1; 

    println!("{s2}");

    let mut s = String::from("hello");
    println!("s before {s}");
    // free s->"hello" string block
    // call drop on the "hello" then assign s to "ahoy"
    // std::mem::drop(s); // added automatically by rust compiler
    s = String::from("ahoy");
    println!("s after: {s} the original s is also freed");

    let mut s3 = String::from("henlo");
    let s4 = s3.clone();
    s3.push_str(" :D");
    println!("s4 clones s3: {s4}");
    println!("s3 is now after push str: {s3}");

    takes_ownership(s3);
    println!("s3 is invalid now");

    let x = 42;
    //Because i32 implements the Copy trait,
    // x does NOT move into the function,
    // so it's okay to use x afterward.
    makes_copy(x);
    println!("x is still valid {x}");

    let mut s1 = gives_ownership();
    s1 = takes_and_gives_back(s1);
    println!("s3: {s1}");

    // multiple return values
    // case of moving in then moving out
    let (s1, s1_len) = calculate_length(s1);
    print!("the length of s1{s1} is {s1_len}");

}

fn takes_ownership(s: String) {
    println!("this function now owns s3 {s}");
}

fn makes_copy(some_integer: u32) {
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    return some_string;
}

fn takes_and_gives_back(s: String) -> String {
    println!("i own this string for now until i return it back");
    return s;
}

// demonstrate if you want multiple return values
fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();
    return (s, len);
}
