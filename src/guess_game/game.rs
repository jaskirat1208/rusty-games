use crate::guess_game::board;
use crate::guess_game::players;
use crate::traits;

type PlayerBox = Box<dyn traits::player_traits::Player>;

pub enum Level {
    Easy,
    Hard,
}

pub struct GuessingGameState {
    pub id: u8, // who played the move
    pub board_response: board::BoardResponse,
}

pub struct GuessingGame {
    board: board::GuessingGameBoard,
    players: u8,
    last_player_move: u8,
    player_props: Vec<PlayerBox>,
}

impl GuessingGame {
    /// Returns a new Game Object. This object is passed on to the core
    /// engine who simulates the game play
    ///
    ///
    /// - **players** : No of players to play this game
    pub fn new(players: u8, humans: u8, level: Level) -> GuessingGame {
        let mut player_props: Vec<PlayerBox> = Vec::new();

        for i in 1..players + 1 {
            if i <= humans {
                println!("Adding Human player with id: {}", i);
                let name = format!("Human {}", i.to_string());
                player_props.push(Box::new(players::human::HumanPlayer::new(i as i32, name)));
            } else {
                println!("Adding bot with id: {}", i);
                let name = format!("Computer {}", i);
                match level {
                    Level::Easy => {
                        let bot = players::random::ComputerEasy::new(i as i32, name);
                        player_props.push(Box::new(bot));
                    }
                    Level::Hard => {
                        player_props.push(Box::new(players::optimal::ComputerHard::new(
                            i as i32, name,
                        )));
                    }
                }
            }
        }

        GuessingGame {
            board: board::GuessingGameBoard::new(),
            players,
            last_player_move: players, // Initializing it to last so that it starts from player 1
            player_props,
        }
    }

    /**
     * All players(human and AI) are specified upfront into a vector. This will
     * register them and return the game object.
     */
    pub fn new_w_custom_bots(bots: Vec<PlayerBox>) -> GuessingGame {
        let players = bots.len() as u8;
        GuessingGame {
            board: board::GuessingGameBoard::new(),
            players,
            last_player_move: players,
            player_props: bots,
        }
    }
}

impl GuessingGame {
    fn get_player(&self, idx: u8) -> &PlayerBox {
        &self.player_props[idx as usize - 1]
    }

    /// Checks if the move is valid. If invalid, asks the player to move again,
    /// otherwise returns the result and forwards it to the handlers of all players.
    fn simulate(&mut self, turn: String) {
        if self.board.is_valid(turn.to_string()) {
            let board_response = self.board.update(turn);
            let state = traits::player_traits::Turn::GuessingGame(GuessingGameState {
                id: self.last_player_move,
                board_response,
            });
            for player in self.player_props.iter_mut() {
                player.update_game_state(&state);
            }
            self.last_player_move = (self.last_player_move) % self.players + 1;
        }
    }
}

impl traits::game_traits::Start for GuessingGame {
    /// Initialize the state. Sets the board state so that game can be played
    fn init(&mut self) {
        self.board.init();
    }
}

impl traits::game_traits::Update for GuessingGame {
    /// Player plays the next move. Board is updated after every move.
    /// Every move is just a command line read operation
    fn update(&mut self) {
        println!(
            "Player {}'s turn",
            (self.last_player_move % self.players) + 1
        );

        // Returns the Players enum object
        let player = self.get_player((self.last_player_move % self.players) + 1);

        let move_played = (*player).play();
        println!("Player {} played: {}", (*player).name(), move_played);
        self.simulate(move_played);
    }
}

impl traits::game_traits::Terminate for GuessingGame {
    /// Returns true if the game is over. This game will never result in a tie.
    /// So, skipping the case of tie, in a custom game, you can also add a
    /// handler for tie. This is an indication for the engine to call the
    /// `handle_terminate` method
    fn can_terminate(&self) -> bool {
        self.board.terminate()
    }

    /// Behanviour of the game when termination occurs. In this case,
    /// it just says goodbye and exits
    fn handle_terminate(&self) {
        println!(
            "Player {} wins",
            self.get_player(self.last_player_move).name()
        );
        println!("Good game, well played. See ya later. Goodbye");
    }
}
