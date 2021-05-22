mod guess_game;
mod engine;

fn main() {
    let mut game = guess_game::game::GuessingGame::new(3, 2, guess_game::game::Level::Easy);

    engine::start(&mut game);
}
