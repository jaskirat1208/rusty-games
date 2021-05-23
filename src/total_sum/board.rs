use crate::traits;

pub struct BoardResponse {
    pub move_played: i32,
}

pub struct Board {
    /// Required to keep a track if the target variable is reached or not
    m_terminate: bool,

    /// Target Variable
    m_target_sum: i32,
    m_range_min: i32,
    m_range_max: i32,
}

impl traits::board_traits::New<Board> for Board {
    fn new() -> Board {
        Board {
            m_terminate: false,
            m_target_sum: 58,
            m_range_min: 1,
            m_range_max: 10,
        }
    }
}

impl traits::game_traits::Start for Board {
    /// Initialize the state of the game. Sets a target value to reach
    fn init(&mut self) {}
}

impl traits::board_traits::Update<String, BoardResponse> for Board {
    /// Update the state of the game. No updates required as such,
    /// just a response to the move of every player
    fn update(&mut self, move_played: &String, played_by: u8) -> BoardResponse {
        let move_played = self.get_val(move_played.to_string());
        println!("player {} played: {}", played_by, move_played.to_string());

        self.m_target_sum -= move_played;
        println!("Sum left: {}", self.m_target_sum);

        if self.m_target_sum <= 0 {
            if self.m_target_sum < 0 {
                println!("Game tied");
            } else {
                println!("Player {} wins", played_by);
            }
            self.m_terminate = true
        }
        BoardResponse { move_played }
    }

    /// Returns true if a move is valid, otherwise false
    ///
    /// - A move is valid when the input is a positive integer
    fn is_valid(&self, turn: &String) -> bool {
        let val = self.get_val(turn.to_string());
        if val >= self.m_range_min && val <= self.m_range_max {
            true
        } else {
            println!("{} => Invalid move: Please try again", turn);
            false
        }
    }
}

impl traits::game_traits::Terminate for Board {
    /// Returns true if the game is over
    fn can_terminate(&self) -> bool {
        self.m_terminate
    }

    fn handle_terminate(&self) {}
}

impl Board {
    fn get_val(&self, str: String) -> i32 {
        str.trim().parse::<i32>().unwrap_or(-1)
    }
}
