mod guess;
use crate::guess::guess::Guess;
use std::io;
use rand::{thread_rng, Rng};

fn main() {
    println!("Welcome to the Guesser!");
    println!("Type in your guess, which should be a number from 1 to 100.");

    // generate the random number
    let mut rng = thread_rng();
    let random_number = rng.gen_range(1..100);

    // grab user input in the form of a String
    let mut user_input: String = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Invalid input, please re-enter.");

    let user_guess = Guess {
        number: user_input,
    };

    println!("You entered: {}\nValidating...", user_guess.number);
    
    let valid_guess = user_guess.validate_guess();

    if valid_guess == true {
        println!("Guess accepted. Comparing...");
        let user_guess_as_int = user_guess.number.parse::<i32>();
        if user_guess_as_int == Ok(random_number) {
            println!("You guessed correctly!");
        } else {
            println!("Your guess was not correct. Guess again!");
        }
    } else {
        println!("Please enter a number between 1 and 100.");
    }
}