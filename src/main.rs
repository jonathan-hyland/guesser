mod guess;

fn main() {
    println!("Welcome to the Guesser!");
    println!("Type in your guess, which should be a number from 1 to 100.");

    let user_guess = guess::guess::Guess::ask_guess();

    println!("You entered: {}\nValidating...", user_guess.number);
    println!("Guess accepted. Comparing...");

        let mut guessing = true;
        while guessing == true {
            if user_guess.number.trim().parse::<i32>() == Ok(user_guess.rand_num) {
                println!("You guessed correctly!");
                guessing = false;
            } else {
                println!("Your guess was not correct. Guess again!");
                break
            }
        }
    }