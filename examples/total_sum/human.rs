extern crate puzzle_games;

use puzzle_games::engine;
use puzzle_games::game;
use puzzle_games::total_sum;

fn main() {
    let bots = total_sum::players::get_human_players(2);

    let mut game =
        game::Game::<total_sum::board::Board, String, total_sum::board::BoardResponse>::new(bots);
    engine::start(&mut game);
}
