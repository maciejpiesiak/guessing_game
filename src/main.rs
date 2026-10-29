use guessing_game::{get_input, Game};

fn main() {
    let mut game = Game::new();
    game.load_config("./config.json");
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
