#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle::square(40);

    println!(
        "The area of the rect1 is {} square pixels.",
        rect1.area()
    );
    println!(
        "The area of the rect2 is {} square pixels.",
        rect2.area()
    );
}
