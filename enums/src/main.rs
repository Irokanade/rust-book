enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct Coord {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    CurrPos(Coord), // trying out if i want to use real struct
}

// enum can also have methods just like structs
impl Message {
    fn call(&self) {
        println!("I am method of Message");
    }
}

// the following structs could hold the same data as the enum variants hold
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32);

// but if we used the different struct each of which has its own type
// we couldn't easily define a function to take any of these kinds of messages
// as we could with the message enum

fn main() {
    // note there is a standard library for ip
    let home = IpAddr::V4(127, 0, 0, 1);
    if let IpAddr::V4(sec1, sec2, sec3, sec4) = home {
        println!("ipv4 of localhost is {}.{}.{}.{}", sec1, sec2, sec3, sec4);
    }

    let loopback = IpAddr::V6(String::from("::1"));
    if let IpAddr::V6(addr) = loopback {
        println!("ipv6 of localhost is {}", addr);
    }

    let curr_pos = Message::CurrPos(Coord { x: 1, y: 2 });
    // must use conditional like if or match since safe rust want to check the type
    // since in this case the type is known at compile time the compiler may optimize the check away
    // so prefer safe rust until profiler tells otherwise
    if let Message::CurrPos(coord) = &curr_pos {
        println!(
            "curr_pos is CurrPos type with x: {}, y: {}",
            coord.x, coord.y
        );
    }

    curr_pos.call();

    // rust does not have null
    // but uses option
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }
    let some_number = Option::Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; // won't compile
}
