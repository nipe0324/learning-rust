use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess_number: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("invalid input: {guess}");
                continue;
            }
        };

        println!("You guessed: {guess_number}");

        match guess_number.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Your win!");
                break;
            }
        };
    }
}
