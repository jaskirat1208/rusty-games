use rand::Rng;
use std::cmp::Ordering;

pub struct GuessingGameBoard {
    /// Required to keep a track if the target variable is reached or not
    m_terminate: bool,

    /// Target Variable
    m_target: i32,
}

impl GuessingGameBoard {
    pub fn new() -> GuessingGameBoard {
        let game = GuessingGameBoard {
            m_terminate: false,
            m_target: 0,
        };
        return game;
    }
}

impl GuessingGameBoard {
    /// Initialize the state of the game. Sets a target value to reach
    pub fn init(&mut self) {
        self.m_target = rand::thread_rng().gen_range(1..100);
    }

    /// Update the state of the game. No updates required as such,
    /// just a response to the move of every player
    pub fn update(&mut self, guess: &String) {
        let guess = self.get_val(guess);

        match guess.cmp(&self.m_target) {
            Ordering::Equal => self.handle_equal(),
            Ordering::Less => self.handle_less(),
            Ordering::Greater => self.handle_greater(),
        }
    }

    /// Returns true if the game is over
    pub fn terminate(&self) -> bool {
        return self.m_terminate;
    }

    /// Returns true if a move is valid, otherwise false
    ///
    /// - A move is valid when the input is a positive integer
    pub fn is_valid(&self, turn: &String) -> bool {
        if self.get_val(turn) > 0 {
            return true;
        }
        println!("Invalid move: Please try again");
        return false;
    }
}

impl GuessingGameBoard {
    fn get_val(&self, str: &String) -> i32 {
        match str.trim().parse::<i32>() {
            Ok(num) => {
                return num;
            }
            Err(_) => {
                return -1;
            }
        };
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
