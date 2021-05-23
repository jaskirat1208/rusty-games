use crate::traits;

/// Starts the engine. An infinite while loop which breaks when can_terminate function
/// of the game is called
pub fn start<
    T: traits::game_traits::Start + traits::game_traits::Update + traits::game_traits::Terminate,
>(
    game: &mut T,
) {
    game.init();
    loop {
        game.update();
        if game.can_terminate() {
            game.handle_terminate();
            break;
        }
    }
}
