mod guess;
use crate::guess::guess::Guess;

fn main() {
    println!("Welcome to the Guesser!");
    println!("Type in your guess, which should be a number from 1 to 100.");

    let user_input: String = "101".to_string();

    let user_guess = Guess {
        number: user_input,
    };

    println!("You entered: {}\nValidating...", user_guess.number);
    
    let valid_guess = user_guess.validate_guess();

    if valid_guess == true {
        println!("Guess accepted. Comparing...");
    } else {
        println!("Please enter a number between 1 and 100.");
    }
}