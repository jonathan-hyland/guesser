pub mod guess_functions;

pub mod guess {

    pub use crate::guess::guess_functions;

    #[derive(PartialOrd, PartialEq)]

    pub struct Guess {
        pub number: String,
        pub rand_num: i32,
    }

    impl Guess {

        pub fn new_guess() {
            let mut user_guess: Guess;  
            let mut guessing = true;
            let random_number = guess_functions::guess_functions::gen_rand_num();

            while guessing == true {
                let user_input = guess_functions::guess_functions::ask_guess();
                let valid_guess = guess_functions::guess_functions::validate_guess(user_input.to_string());

                if valid_guess == true {
                    user_guess = Guess {
                        number: user_input,
                        rand_num: random_number,
                    };
                        if user_guess.number.trim().parse::<i32>() == Ok(user_guess.rand_num) {
                            println!("Your guess was correct!");
                            guessing = false;
                        } else {
                            println!("Wrong guess. Guess again!");
                            guessing = true;
                        }
                } else {
                    guessing = true;
                };
            };
        }
    }
}