mod guess;

fn main() {
    println!("Welcome to the Guesser!");
    println!("Type in your guess, which should be a number from 1 to 100.");
    guess::guess::Guess::new_guess();
}