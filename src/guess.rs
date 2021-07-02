pub mod guess_functions;

pub mod guess {

    use std::io;
    pub use crate::guess::guess_functions;

    #[derive(PartialOrd, PartialEq, Clone)]

    pub struct Guess {
        pub number: String,
        pub rand_num: i32,
    }

    impl Guess {

        pub fn ask_guess() -> Guess {
            let mut user_guess: Guess = Guess {
                                        number: String::new(),
                                        rand_num: 0,
                                        };
            let mut guessing = true;
            while guessing == true {
                let mut user_input: String = String::new();
                io::stdin()
                    .read_line(&mut user_input)
                    .expect("Invalid input, please re-enter.");

                let random_number = guess_functions::guess_functions::gen_rand_num();
                println!("{}", random_number);
                    
                let valid_guess = guess_functions::guess_functions::validate_guess(user_input.trim().to_string());

                if valid_guess == true {
                    user_guess = Guess {
                        number: user_input,
                        rand_num: random_number,
                    };
                    guessing = false;
                } else {
                    user_guess = Guess {
                        number: user_input,
                        rand_num: random_number,
                    };
                    guessing = true;
                };
            };
        user_guess
        }
    }
}