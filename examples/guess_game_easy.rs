use guessing_game::engine;
use guessing_game::guess_game;

fn main() {
    let mut game = guess_game::game::GuessingGame::new(3, 1, guess_game::game::Level::Easy);

    engine::start(&mut game);
}
