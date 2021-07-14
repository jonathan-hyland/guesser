use guess::Guess;
use std::io;

fn main() {
    let mut the_game = true;

    while the_game == true {
        println!("Welcome to the Guesser!");
        println!("Type in your guess, which should be a number from 1 to 100.");
        
        let guessing_game = Guess::new_guess();

        if guessing_game == true {
            let mut new_game = String::new();
            let mut initialize_game = false;

            while initialize_game == false {
                println!("The game is over! Would you like to play again? y/n");
                io::stdin()
                    .read_line(&mut new_game)
                    .expect("Input could not be read.");
                    if new_game.trim() == "y" {
                        initialize_game = true;
                        the_game = true;
                    } else if new_game.trim() == "n" {
                        initialize_game = true;
                        the_game = false;
                        println!("Thanks for playing!");
                    } else {
                        new_game.clear();
                        initialize_game = false;
                        println!("Please enter y or n.");
                    }
            }
        } else {
            the_game = false;
            println!("Thanks for playing!");
        }
    }
}