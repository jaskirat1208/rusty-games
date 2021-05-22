use crate::guess_game;

pub fn start<T: guess_game::Start + guess_game::Update + guess_game::Terminate>(game: &mut T) {
    // let mut game = Game<GuessGameBoard, GuessGamePlayer>::new(3/**players*/);
    game.init();
    loop {
        game.update();
        if game.can_terminate() {
            game.handle_terminate();
            break;
        }
    }
}
