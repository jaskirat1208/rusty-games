pub mod human;
pub mod optimal;
pub mod random;

use crate::guess_game;

pub trait Player: guess_game::Play + guess_game::UpdateGameState + guess_game::Name {}
