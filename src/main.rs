mod guess_game;
mod engine;

fn main() {
    let mut board = guess_game::game::GuessingGame::new(3);

    engine::start(&mut board);
}
