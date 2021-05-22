use puzzle_games::engine;
use puzzle_games::guess_game;

fn main() {
    let mut game = guess_game::game::GuessingGame::new(2, 2, guess_game::game::Level::Hard);

    engine::start(&mut game);
}