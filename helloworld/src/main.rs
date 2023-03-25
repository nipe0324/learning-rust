use std::num::ParseIntError;

fn main() {
    println!("Hello, world!");
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

fn parse_and_plus(s: &str, n: i32) -> Result<i32, ParseIntError> {
    let parsed_int = s.parse::<i32>()?;
    Ok(parsed_int + n)
}

#[test]
fn parse_and_plus_test() {
    assert_eq!(parse_and_plus("100", 1).unwrap(), 101);
    assert!(parse_and_plus("abc", 1).is_err());
}
