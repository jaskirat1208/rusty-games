mod guess_game_board;
mod guessing_game;
mod engine;

fn main() {
    let mut board = guessing_game::GuessingGame::new(3);

    engine::start(&mut board);
}
