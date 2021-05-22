mod engine;
mod guess_game;

fn main() {
    let mut game = guess_game::game::GuessingGame::new(3, 1, guess_game::game::Level::Hard);

    engine::start(&mut game);
}
