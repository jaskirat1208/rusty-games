use crate::traits;
use std::io;

pub struct HumanPlayer {
    m_name: String,
    _m_id: i32,
}

pub fn read() -> String {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    guess
}

impl HumanPlayer {
    pub fn new(id: i32, name: String) -> HumanPlayer {
        HumanPlayer {
            m_name: name,
            _m_id: id,
        }
    }
}

impl traits::player_traits::Name for HumanPlayer {
    fn name(&self) -> String {
        self.m_name.to_string()
    }
}

impl traits::player_traits::Play for HumanPlayer {
    fn play(&self) -> String {
        read()
    }
}

impl traits::player_traits::UpdateGameState for HumanPlayer {}
impl traits::player_traits::Player for HumanPlayer {}
