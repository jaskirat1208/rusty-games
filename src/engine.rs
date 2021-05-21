use crate::guessing_game;

pub fn start(game: & mut guessing_game::GuessingGame) {
    // let mut game = Game<GuessGameBoard, GuessGamePlayer>::new(3/**players*/);
    game.init();
    loop {
        game.update();
        if game.terminate() {
            // game.handleTerminate();
            break;
        }
    }
}