use crate::traits;
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

impl traits::player_traits::Name for ComputerEasy {
    fn name(&self) -> String {
        self.m_name.to_string()
    }
}

impl traits::player_traits::Play<String> for ComputerEasy {
    fn play(&self) -> String {
        rand::thread_rng().gen_range(1..100).to_string()
    }
}

impl traits::player_traits::UpdateGameState<traits::player_traits::Turn> for ComputerEasy {}
impl traits::player_traits::Player<String, traits::player_traits::Turn> for ComputerEasy {}
