mod engine;
mod guess_game;

fn main() {
    let mut game = guess_game::game::GuessingGame::new(3, 2, guess_game::game::Level::Easy);

    engine::start(&mut game);
}
