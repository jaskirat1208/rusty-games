use crate::traits;
use rand::Rng;
use std::cmp::Ordering;

pub struct BoardResponse {
    // TODO(Remove result. Let the AI compute the mapping himself.
    // THe move object should be the template Move object specified in the game)
    pub move_played: i32,
    pub result: Ordering,
    pub played_by: u8,
}

pub struct GuessingGameBoard {
    /// Required to keep a track if the target variable is reached or not
    m_terminate: bool,

    /// Target Variable
    m_target: i32,
}

impl traits::board_traits::New<GuessingGameBoard> for GuessingGameBoard {
    fn new() -> GuessingGameBoard {
        GuessingGameBoard {
            m_terminate: false,
            m_target: 0,
        }
    }
}

impl traits::game_traits::Start for GuessingGameBoard {
    /// Initialize the state of the game. Sets a target value to reach
    fn init(&mut self) {
        self.m_target = rand::thread_rng().gen_range(1..100);
    }
}

impl traits::board_traits::Update<String, BoardResponse> for GuessingGameBoard {
    /// Update the state of the game. No updates required as such,
    /// just a response to the move of every player
    fn update(&mut self, guess: &String, played_by: u8) -> BoardResponse {
        let guess = self.get_val(guess.to_string());
        println!("player {} played: {}", played_by, guess.to_string());
        let result = guess.cmp(&self.m_target);

        match result {
            Ordering::Less => self.handle_less(),
            Ordering::Greater => self.handle_greater(),
            Ordering::Equal => self.handle_equal(),
        }

        BoardResponse {
            move_played: guess,
            result,
            played_by,
        }
    }

    /// Returns true if a move is valid, otherwise false
    ///
    /// - A move is valid when the input is a positive integer
    fn is_valid(&self, turn: &String) -> bool {
        if self.get_val(turn.to_string()) > 0 {
            true
        } else {
            println!("{} => Invalid move: Please try again", turn);
            false
        }
    }
}

impl traits::game_traits::Terminate for GuessingGameBoard {
    /// Returns true if the game is over
    fn can_terminate(&self) -> bool {
        self.m_terminate
    }

    fn handle_terminate(&self) {}
}

impl GuessingGameBoard {
    fn get_val(&self, str: String) -> i32 {
        str.trim().parse::<i32>().unwrap_or(-1)
    }

    fn handle_less(&self) {
        println!("Too small");
    }

    fn handle_greater(&self) {
        println!("Too big");
    }

    fn handle_equal(&mut self) {
        println!("Victory: You guessed it correct");
        self.m_terminate = true;
    }
}
