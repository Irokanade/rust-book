fn main() {
    let mut s1 = String::from("hello");
    // The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it. 
    // Because the reference does not own it, 
    // the value it points to will not be dropped when the reference stops being used.
    // the action of creating a reference is called borrowing
    let s1_len = calculate_length(&s1);

    println!("the length of {s1} is: {s1_len}");

    change(&mut s1);
    println!("{s1}");

    // Mutable references have one big restriction: 
    // If you have a mutable reference to a value, 
    // you can have no other references to that value.
    let r1 = &mut s1;
    // let r2 = &mut s1;
    println!("{r1}");

    {
        let _r3 = &mut s1;
    } // r3 goes out of scope here, so we can make a new reference with no problems.

    let _r4 = &mut s1;

    // about scope of references
    reference_shenanigans();

    // string slice
    // string slice internally stores ptr, len
    let mut s = String::from("hello world");
    let _hello = &s[0..5];
    let _world = &s[6..11];

    let first_word = first_word(&s);
    // s.clear(); // error due to mutable borrow
    // but immutable borrow in the println!
    println!("first word of {s} is: {}", first_word); 
    println!("first word print again: {first_word}");
    s.clear(); // here is okay as we are done using the reference
    
}

fn calculate_length(s: &str) -> usize {
    return s.len();
}

fn change(s: &mut String) {
    s.push_str(" kitty");
}

fn reference_shenanigans() {
    // Note that a reference’s scope starts from where it is introduced 
    // and continues through the last time that reference is used. 
    // For instance, this code will compile because the last usage 
    // of the immutable references is in the println!, 
    // before the mutable reference is introduced:
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s; // no problem
    println!("{r3}");

    // let reference_to_nothing = dangle();
}

// fn dangle() -> &String {
//     let s = String::from("henlo");
//     &s
// } // Here, s goes out of scope and is dropped, so its memory goes away.

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // returns byte slice

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
