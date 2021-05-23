use crate::traits;
use std::cmp;
use std::cmp::Ordering;

/**
 * Whenever the computer receives info about the turn played by a player,
 * it will try to update the range of the target number. The next move
 * it plays is the mean of this range.
 */
pub struct ComputerHard {
    m_name: String,
    _m_id: i32,
    m_start: i32,
    m_end: i32,
}

impl ComputerHard {
    /**
     * Returns a new computer hard object
     */
    pub fn new(id: i32, name: String) -> ComputerHard {
        ComputerHard {
            m_name: name,
            _m_id: id,
            m_start: 0,
            m_end: 100,
        }
    }
}

impl traits::player_traits::Name for ComputerHard {
    /**
     * Returns the name of the bot.
     */
    fn name(&self) -> String {
        self.m_name.to_string()
    }
}

impl traits::player_traits::Play<String> for ComputerHard {
    /**
     * Plays the next move. Returns the middle value of the range
     * Called on turn.
     */
    fn play(&self) -> String {
        (self.m_start + (self.m_end - self.m_start) / 2).to_string()
    }
}

impl traits::player_traits::UpdateGameState<traits::player_traits::Turn> for ComputerHard {
    /**
     * Updates the state of the game.
     * Called after a player plays its move
     */
    fn update_game_state(&mut self, turn: &traits::player_traits::Turn) {
        match turn {
            traits::player_traits::Turn::GuessingGame(turn) => match turn.board_response.result {
                Ordering::Less => {
                    self.m_start = cmp::max(self.m_start, turn.board_response.move_played) as i32
                }
                Ordering::Greater => {
                    self.m_end = cmp::min(self.m_end, turn.board_response.move_played) as i32
                }
                _ => {
                    self.m_start = turn.id as i32;
                    self.m_end = turn.id as i32;
                }
            },
            _ => {}
        }
    }
}

impl traits::player_traits::Player<String, traits::player_traits::Turn> for ComputerHard {}
