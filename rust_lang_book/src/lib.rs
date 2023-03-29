//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

mod front_of_house;

pub use crate::front_of_house::hosting;

/// Eat at restaurant.
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, rust_lang_book::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
