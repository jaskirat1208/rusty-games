pub mod human;

use crate::total_sum::board;
use crate::traits;

type PlayerBox = traits::player_traits::PlayerBox<String, board::BoardResponse>;

pub fn get_human_players(n: u8) -> Vec<PlayerBox> {
    let mut result: Vec<PlayerBox> = Vec::new();

    for i in 1..n + 1 {
        result.push(Box::new(human::HumanPlayer::new(
            i as i32,
            format!("Human {}", i),
        )))
    }

    result
}
