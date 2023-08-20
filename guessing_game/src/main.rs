use rand::Rng;
use std::io;

fn main() {
    let secret_number = rand::thread_rng()
        .gen_range(1..=100); // Range Syntax: from..=to (including)

    println!("{}----------------", "Guess the number\n");

    println!("The secret number is: {secret_number}");

    println!("Please input your guess: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); //IMPORTANT
                                        //If missing, the Result type may
                                        //  become an Error

    println!("You guessed: {guess}");
}
