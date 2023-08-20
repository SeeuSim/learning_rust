use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100); // Range Syntax: from..=to (including)

    println!("{}----------------", "Guess the number\n");

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); //IMPORTANT
                                            //If missing, the Result type may
                                            //  become an Error

        // expect => Crash with this error
        // match => Provide case by case handling of error (ok, error)

        // let guess:u32 = guess.trim().parse().expect("A number was not input");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Oops! Expected a positive integer, but another string was input.");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
