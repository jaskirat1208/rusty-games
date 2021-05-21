mod guess_game;

fn main() {
    let mut game = guess_game::GuessingGame::new();
    loop {
        game.update();
        if game.terminate() {
            break;
        }
    }
}
