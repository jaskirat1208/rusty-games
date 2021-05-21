// use crate::guess_game_board;
use std::io;

use crate::guess_game_board;

pub struct GuessingGame {

    board: guess_game_board::GuessingGameBoard,
    players: u8,
    last_player_move: u8
}

pub fn read() -> String {

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    return guess;
}

impl GuessingGame {
    /**
     * Returns a new Game Object. This object is passed on to the core engine who simulates the game play
     * 
     * - **players** : No of players to play this game
     */
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
     * Initialize the state. Sets the board state so that game can be played
     */
    pub fn init(&mut self) {
        self.board.init();
    }
    /**
     * Player plays the next move. Board is updated after every move.
     * Every move is just a command line read operation
     */
    pub fn update(&mut self) {
        println!("Player {}'s turn", (self.last_player_move%self.players)+1);
        let turn = read(); 
        if self.board.is_valid(&turn) {
            self.board.update(&turn);
            self.last_player_move = (self.last_player_move)%self.players + 1;
        }
        
    }

    /**
     * Returns true if the game is over. This game will never result in a tie.
     * So, skipping the case of tie, in a custom game, you can also add a handler for 
     * tie.
     */
    pub fn terminate(&self) -> bool {
        if self.board.terminate() {
            println!("Good game. Well played. Player: {} wins", self.last_player_move);
            return true;
        }
        return false;
    }
}