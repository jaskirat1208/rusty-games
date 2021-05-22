use crate::guess_game;
use std::cmp::Ordering;

pub struct ComputerHard {
    m_name: String,
    _m_id: i32,
    m_start: i32,
    m_end: i32,
}

impl ComputerHard {
    pub fn new(id: i32, name: &String) -> ComputerHard {
        ComputerHard {
            m_name: name.to_string(),
            _m_id: id,
            m_start: 0,
            m_end: 100,
        }
    }
}

impl ComputerHard {
    pub fn name(&self) -> String {
        self.m_name.to_string()
    }
}

impl guess_game::Play for ComputerHard {
    fn play(&self) -> String {
        (self.m_start + (self.m_end - self.m_start) / 2).to_string()
    }
}

impl guess_game::UpdateGameState for ComputerHard {
    fn update_game_state(&mut self, turn: &guess_game::Turn) {
        match turn {
            guess_game::Turn::GuessingGame(turn) => match turn.board_response.result {
                Ordering::Less => self.m_start = turn.board_response.move_played as i32,
                Ordering::Greater => self.m_end = turn.board_response.move_played as i32,
                _ => {
                    self.m_start = turn.id as i32;
                    self.m_end = turn.id as i32;
                }
            },
        }
    }
}
