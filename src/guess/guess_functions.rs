pub mod guess_functions {
    
    use rand::{thread_rng, Rng};
    use std::io;

    pub fn ask_guess() -> String {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Invalid input, please re-enter.");
        user_input
    }

    pub fn gen_rand_num() -> i32 {
        let mut rng = thread_rng();
        let random_number = rng.gen_range(1..100);
        random_number
    }

    pub fn validate_guess(user_input: String) -> bool {
        // check to ensure that the user input is a number (adapted from https://stackoverflow.com/questions/30355185/how-to-read-an-integer-input-from-the-user-in-rust-1-0)
        let user_number = user_input.trim();
        match user_number.parse::<i32>() {
            Ok(i) => if i > 100 || i < 1 {
                        false
                    } else {
                        true
                    }
            Err(..) => { println!("This was not an integer: {}", user_number);
                        false }
                    }
    }
}