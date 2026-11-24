// rust struct does not garantee field order stored like c
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuple structs
// Tuple structs are useful when you want to give the whole tuple a name 
// and make the tuple a different type from other tuples
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like structs
// useful when you need to implement a trait on some type
// but don’t have any data that you want to store in the type itself.
struct AlwaysEqual;

fn main() {
    // Note that the entire instance must be mutable;
    // Rust doesn’t allow us to mark only certain fields as mutable.
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = build_user(String::from("user2@rust.com"), String::from("user2"));

    // struct update syntax
    let user3 = User {
        email: String::from("user3"),
        ..user2
    };

    // can i still own user 2 even though some of its string are borrowed
    // apprently no due to partially moved user2
    // let stolen_user = user2;

    // wow using struct udpate uses '=' operator
    // user2 stuff is moved into user3
    // println!("is user2 username still valid? {}", user2.username);
    // email is not moved into user3 so email is still valid
    println!("what about user2 email? {}", user2.email);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black({}, {}, {})", black.0, black.1, black.2);

    // Unlike tuples, tuple structs require you to name the type of the struct when you destructure them
    let Point(x, y, z) = origin;
    println!("origin({}, {}, {})", x, y, z);

    // unlike c++ which requires empty struct to be nonzero
    // rust unit-like structs are size 0
    let subject = AlwaysEqual;

}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}
