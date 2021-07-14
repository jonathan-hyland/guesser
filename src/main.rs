mod guess;
use crate::guess::guess::Guess;

fn main() {
    let mut the_game = true;

    while the_game == true {
        println!("Welcome to the Guesser!");
        println!("Type in your guess, which should be a number from 1 to 100.");
        
        let new_game = Guess::new_guess();

        if new_game == true {
            let replay = Guess::play_again();

            if replay == true {
                the_game = true;
            } else {
                the_game = false;
                println!("Thanks for playing!");
            }
        } 
    }
}