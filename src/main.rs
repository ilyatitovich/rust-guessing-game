// Standart library for input/output
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // mut - means that varaible mutable and can change in the future
    // String::new() - means that varaible store string
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)  // handling user's input, using pointer
        .expect("Failed to read line"); // handlign Errors

    println!("You guessed: {}", guess);
}