#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // any function declared here are called associated functions

    fn square(size: u32) -> Self {
        Self { 
            width: size, 
            height: size 
        }
    }

    // functions that take self are called methods
    fn area(self: &Self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn set_height(self: &mut Self, height: u32) {
        self.height = height;
    }

    fn steal(self: Self) {
        println!("i've stolen self and self is now invalid state after calling this function");
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        area(&rect1)
    );

    //  Putting the specifier :? inside the curly brackets 
    // tells println! we want to use an output format called Debug
    println!("rect1 is {rect1:?}");

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    // dbg! takes ownership
    // dbg! also prints to stderr
    let scale = 2;
    let mut rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    rect2.set_height(55);

    // We don’t want dbg! to take ownership of rect1 
    // so we use a reference to rect1 in the next call.
    dbg!(&rect2);

    println!("The area of the rect2 is {} square pixels", rect2.area());
    rect2.steal();
    // let area = rect2.area();

    // Here’s how it works: When you call a method with object.something(),
    // Rust automatically adds in &, &mut, or * 
    // so that object matches the signature of the method. 
    // In other words, the following are the same:
    // p1.distance(&p2);
    // (&p1).distance(&p2);

    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect4 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect5 = Rectangle {
        width: 60,
        height: 45,
    };


    println!("Can rect3 hold rect4? {}", rect3.can_hold(&rect4));
    println!("Can rect3 hold rect5? {}", rect3.can_hold(&rect5));

    let _sq1 = Rectangle::square(30);

}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
