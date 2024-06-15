use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");
    // in rust, vars are immutable by default
    let mut guess = String::new();

    io::stdin()
        // references are mutable by default as well
        .read_line(&mut guess)
        .expect("Failed to read line"); // try compiling without this line

    println!("You guessed: {}", guess);

    let x = 5;
    let y = 10;

    println!("x = {} and y + 2 = {}",x , y + 2);

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    
    loop {
        println!("Please input your guess.");
        
        let mut gues = String::new();
        
        io::stdin()
        .read_line(&mut gues)
        .expect("Failed to read line");
        
        let gues: u32 = match gues.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please input number");
                continue;
            }
        };
        // let gues: gues.trim().parse().expect("Please type a number!");
        println!("You guessed: {gues}");
    
        match gues.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
    
}