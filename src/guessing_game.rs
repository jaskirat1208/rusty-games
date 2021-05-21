// use crate::guess_game_board;
use std::io;

use crate::guess_game_board;

pub struct GuessingGame {
    board: guess_game_board::GuessingGameBoard,
    players: u8,
    last_player_move: u8
}

pub fn read() -> i32 {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter an integer 0 to 100");
            return -1;
        },
    };

    return guess;
}

impl GuessingGame {
    pub fn new(players: u8) -> GuessingGame {
        return GuessingGame {
            board: guess_game_board::GuessingGameBoard::new(),
            players: players,
            last_player_move: players       // Initializing it to last so that it starts from player 1
        }
    }
}

impl GuessingGame {
    /**
     * Initialize the state of the game
     */
    pub fn init(&mut self) {
        self.board.init();
    }
    /**
     * Update the state of the game
     */
    pub fn update(&mut self) {
        println!("Player {}'s turn", (self.last_player_move%self.players)+1); 
        self.board.update(read());
        self.last_player_move = (self.last_player_move)%self.players + 1;
    }

    /**
     * Condition for termination of the event loop
     */
    pub fn terminate(&self) -> bool {
        if self.board.terminate() {
            println!("Good game. Well played. Player: {} wins", self.last_player_move);
            return true;
        }
        return false;
    }
}