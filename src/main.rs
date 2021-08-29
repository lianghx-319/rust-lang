#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            length: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        length: 50,
    };
    let rect1 = Rectangle {
        width: 10,
        length: 40,
    };
    let rect2 = Rectangle {
        width: 35,
        length: 55,
    };
    let s = Rectangle::square(20);
    println!("{}", s.area());

    println!("{}", rect1.can_hold(&rect2));

    println!("{}", rect2.can_hold(&rect));
}
