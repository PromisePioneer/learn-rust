use log::log;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // ini seperti constructor
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    let rectangle2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rectangle3 = Rectangle {
        width: 60,
        height: 45,
    };

    if rectangle.width() {
        println!("The value of rectangle width is nonzero");
    }

    println!("The value of area rectangle is {}", rectangle.area());

    println!(
        "Can rectangle hold rectangle2, {}",
        rectangle.can_hold(&rectangle2)
    );
    println!(
        "Can rectangle hold rectangle3, {}",
        rectangle.can_hold(&rectangle3)
    );

    println!("{:?}", Rectangle::square(3));
}
