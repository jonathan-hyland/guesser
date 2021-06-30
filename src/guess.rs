// Container for Guess struct and related methods

pub mod guess {
    #[derive(PartialOrd, PartialEq)]

    pub struct Guess {
        pub number: i32,
    }

    impl std::fmt::Display for Guess {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self)
          }
    }

    impl Guess {
        // method to ensure that the user provided number falls within the specified range
        pub fn validate_guess(&self) -> bool {
            if self.number > 100 {
                false
            } else if self.number < 1 {
                false
            } else {
                true
            }
        }
    }
}
