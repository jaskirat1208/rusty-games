use crate::guess_game;
use rand::Rng;

pub struct ComputerEasy {
    m_name: String,
    _m_id: i32,
}

impl ComputerEasy {
    pub fn new(id: i32, name: String) -> ComputerEasy {
        ComputerEasy {
            m_name: name,
            _m_id: id,
        }
    }
}

impl guess_game::Name for ComputerEasy {
    fn name(&self) -> String {
        self.m_name.to_string()
    }
}

impl guess_game::Play for ComputerEasy {
    fn play(&self) -> String {
        rand::thread_rng().gen_range(1..100).to_string()
    }
}

impl guess_game::UpdateGameState for ComputerEasy {}
impl guess_game::players::Player for ComputerEasy {}
