// Container for Guess struct and related methods


pub mod guess {
    #[derive(PartialOrd, PartialEq)]

    pub struct Guess {
        pub number: String,
    }

    impl std::fmt::Display for Guess {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self)
          }
    }

    impl Guess {
        // method to ensure that the user provided a number and that number falls within the specified range
        pub fn validate_guess(&self) -> bool {
            // check to ensure that the user input is a number (adapted from https://stackoverflow.com/questions/30355185/how-to-read-an-integer-input-from-the-user-in-rust-1-0)
            let user_number = self.number.trim();
            match user_number.parse::<i32>() {
                Ok(i) => if i > 100 || i < 1 {
                            false
                        } else {
                            true
                        }
                Err(..) => { println!("this was not an integer: {}", user_number);
                            false }
            }
        }
    }
}
