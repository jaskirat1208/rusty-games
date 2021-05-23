use puzzle_games::engine;
use puzzle_games::guess_game;
use puzzle_games::game;

fn main() {
    let bots = guess_game::players::get_computer_players(2, game::Level::Hard);

    let mut game = game::Game::<
        guess_game::board::GuessingGameBoard,
        String,
        guess_game::board::BoardResponse,
    >::new(bots);
    engine::start(&mut game);
}
