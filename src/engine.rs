use crate::guess_game;

pub fn start(game: & mut guess_game::game::GuessingGame) {
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