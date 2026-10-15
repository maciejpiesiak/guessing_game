//! # My Guessing Game implementation
//!
//! `guessing-game` is a simple game. The aim is to guess a number. Player has
//! <number tries> tries and a goal is to guess a number that is somewhere
//! between <min>..<max>

use rand;
use std::env;
use std::error::Error;
use std::io;
use std::process;

// default number of tries used by default
const DEFAULT_NUMBER_OF_TRIES: usize = 3;
const DEFAULT_SECRET_MINIMUM: usize = 1;
const DEFAULT_SECRET_MAXIMUM: usize = 10;

pub struct Game {
    // the main struct that holds configuration
    // only number of tries is publicly available
    pub tries: usize,
    secret_number: usize,
    pub guesses: Vec<usize>,
}

impl Game {
    pub fn new() -> Game {
        // number of command-line arguments:
        // 0 => use all default values
        // 1 => use default value for minimum and maximum threshold
        // 2 => use default value for maximum threshold
        // 3 => use values all command-line arguments
        // exit (error code 1 or 2) if contract violated
        let provided_args: Vec<String> = env::args().collect();

        if provided_args.len() > 4 {
            eprintln!("Provided too many arguments..");
            process::exit(1);
        };

        let tries: usize = match provided_args.get(1) {
            Some(arg1) => arg1
                .trim()
                .parse()
                .expect("Error while parsing number of tries argument"),
            None => DEFAULT_NUMBER_OF_TRIES,
        };
        let secret_minimum: usize = match provided_args.get(2) {
            Some(arg2) => arg2
                .trim()
                .parse()
                .expect("Error while parsing minimum threshold argument"),
            None => DEFAULT_SECRET_MINIMUM,
        };
        let secret_maximum: usize = match provided_args.get(3) {
            Some(arg2) => arg2
                .trim()
                .parse()
                .expect("Error while parsing maximum threshold argument"),
            None => DEFAULT_SECRET_MAXIMUM,
        };

        if secret_minimum > secret_maximum {
            eprintln!("Minimum threshold should be lower than maximum");
            process::exit(2);
        };
        let secret_number: usize = rand::random_range(secret_minimum..=secret_maximum);
        let guesses: Vec<usize> = Vec::new();
        Game {
            secret_number,
            tries,
            guesses,
        }
    }

    pub fn is_correct(&mut self, guess: usize) -> bool {
        // check against 3 scenarios:
        // 1. provided number is greater than a secret number
        // 2. provided number is lower than the secret number
        // 3. provided number equals the secret number
        self.guesses.push(guess);
        match guess {
            g if g > self.secret_number => {
                println!("Too much..");
                false
            }
            g if g < self.secret_number => {
                println!("Little bit more..");
                false
            }
            _ => {
                println!("You win!");
                true
            }
        }
    }
}

pub fn get_input() -> Result<usize, Box<dyn Error>> {
    // get user's guess or bring back an error message in case of any troubles
    // while parsing String to usize
    let mut guess: String = String::new();
    io::stdin().read_line(&mut guess)?;
    let guess: usize = guess.trim().parse()?;
    Ok(guess)
}

#[cfg(test)]
mod tests {

    #[test]
    fn check_game() {
        // simple unit test to check wrong/correct guesses
        todo!();
    }
}
