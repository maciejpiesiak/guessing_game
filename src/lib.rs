//! # My Guessing Game implementation
//!
//! `guessing-game` is a simple game. The aim is to guess a number. Player has
//! <number tries> tries and a goal is to guess a number that is somewhere
//! between <min>..<max>

use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::Read;
use std::process;

// default number of tries used by default
const CONFIGURATION_PATH: &str = "./config.json";

#[derive(Serialize, Deserialize)]
struct Config {
    tries: usize,
    minimum: usize,
    maximum: usize,
}

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
        // 0 => use default configuration file path
        // 1 => use custom configuration file path
        // exit (error code 1) if contract violated
        let provided_args: Vec<String> = env::args().collect();

        if provided_args.len() > 2 {
            eprintln!("Provided too many arguments..");
            process::exit(1);
        };

        let path: &str = match provided_args.get(1) {
            Some(arg) => arg.trim(),
            None => CONFIGURATION_PATH,
        };

        let mut config_contents: String = String::new();
        let mut config_file =
            File::open(path).expect("Configuration file: Configuration file not found");
        config_file
            .read_to_string(&mut config_contents)
            .expect("Configuration file: Could not read content");
        let config: Config = serde_json::from_str(&config_contents)
            .expect("Configuration file: Found file with unknown configuration scheme");

        if config.minimum > config.maximum {
            eprintln!("Minimum threshold should be lower than maximum");
            process::exit(2);
        };

        let secret_number: usize = rand::random_range(config.minimum..=config.maximum);
        let tries: usize = config.tries;
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
