pub mod guess_functions;

pub mod guess {
    use crate::guess::guess_functions::guess_functions::*;
    use std::io;

    #[derive(PartialOrd, PartialEq)]

    pub struct Guess {
        pub number: String,
        pub rand_num: i32,
    }

    impl Guess {

        pub fn new_guess() -> bool {
            let mut finished = false;
            let mut user_guess: Guess;  
            let mut guessing = true;
            let random_number = gen_rand_num();

            // For testing - gives the generated random number to use
            println!("{}", random_number);

            while guessing == true {
                let user_input = ask_guess();
                let valid_guess = validate_guess(user_input.to_string());

                if valid_guess == true {
                    user_guess = Guess {
                        number: user_input,
                        rand_num: random_number,
                    };
                        if user_guess.number.trim().parse::<i32>() == Ok(user_guess.rand_num) {
                            println!("Your guess was correct!");
                            guessing = false;
                            finished = true;
                        } else {
                            println!("Wrong guess. Guess again!");
                            guessing = true;
                        }
                } else {
                    guessing = true;
                };
            };
        finished
        }

        pub fn play_again() -> bool {
            let mut new_game = String::new();
            let mut replay = false;
            let mut initialize_game = false;

            while initialize_game == false {
                println!("The game is over! Would you like to play again? y/n");
                io::stdin()
                    .read_line(&mut new_game)
                    .expect("Input could not be read.");
                    if new_game.trim() == "y" {
                        replay = true;
                        initialize_game = true;
                    } else if new_game.trim() == "n" {
                        replay = false;
                        initialize_game = true;
                    } else {
                        new_game.clear();
                        initialize_game = false;
                        println!("Please enter y or n.");
                    };
            }
            replay
        }
    }
}