use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");
    // in rust, vars are immutable by default
    let mut guess = String::new();

    io::stdin()
        // references are mutable by default as well
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}