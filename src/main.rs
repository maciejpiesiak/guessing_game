use guessing_game::{Game, get_input};

fn main() {
    let mut game = Game::new();
    for _ in 1..=game.tries {
        let guess = loop {
            match get_input() {
                Ok(value) => {
                    break value;
                }
                Err(_) => {
                    println!("Ups.. Not a number. Try again.");
                    continue;
                }
            };
        };
        if game.is_correct(guess) == true {
            break;
        }
    }
}
