/*
    The standard library is the set of features that Rust gives you out of the box.
    It’s packaged with the Rust language for your use.
    The standard library includes common Rust types, like vectors, strings, and hash maps,
    as well as traits that represent common behavior, like the iterator trait that enables
    you to use .iter() on vectors to loop through them.
    The standard library also includes Rust’s built-in macros, the core types and traits in Rust,
    the standard library API reference, and the Rust Prelude, which is a small set of items
    that Rust automatically imports into every Rust program.
    
     std::io gives you the ability to accept user input and print text to the screen
 */
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    // created a mutable variable that is currently bound to a new, empty instance of a String
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); // crashes the program if an error occurs

    println!("You guessed: {}", guess); // {} is a placeholder for the value of the variable
}
