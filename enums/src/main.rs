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

fn main() {
    // note there is a standard library for ip
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

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
}
