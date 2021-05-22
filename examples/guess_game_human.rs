use guessing_game::engine;
use guessing_game::guess_game;

fn main() {
    let mut game = guess_game::game::GuessingGame::new(2, 2, guess_game::game::Level::Hard);

    engine::start(&mut game);
}
