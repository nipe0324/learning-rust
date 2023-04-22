use std::num::ParseIntError;

fn main() {
    println!("Hello, world!");
    println!("1 + 2 = {}", plus(1, 2));
}

fn plus(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn plus_test() {
    assert_eq!(plus(4, 5), 9);
    assert_eq!(plus(100, -1), 99);
    assert_eq!(plus(114000, 514), 114514);
}
