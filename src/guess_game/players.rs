pub mod human;
pub mod optimal;
pub mod random;

use crate::game;
use crate::guess_game;
use crate::traits;

type PlayerBox = traits::player_traits::PlayerBox<String, guess_game::board::BoardResponse>;

pub fn get_computer_players(n: u8, level: game::Level) -> Vec<PlayerBox> {
    let mut result: Vec<PlayerBox> = Vec::new();
    for i in 1..n + 1 {
        match level {
            game::Level::Easy => {
                result.push(Box::new(random::ComputerEasy::new(
                    i as i32,
                    format!("Computer {}", i),
                )));
            }
            game::Level::Hard => {
                result.push(Box::new(optimal::ComputerHard::new(
                    i as i32,
                    format!("Computer {}", i),
                )));
            }
        }
    }
    result
}

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
