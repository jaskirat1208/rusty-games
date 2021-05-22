use crate::guess_game::board;
use crate::guess_game::players;
use crate::guess_game::Play;
use crate::guess_game::UpdateGameState;

pub enum Level {
    Easy,
    Hard,
}

pub struct GuessingGameState {
    pub id: u8, // who played the move
    pub board_response: board::BoardResponse,
}

enum Players {
    Human(players::human::HumanPlayer),
    ComputerEasy(players::random::ComputerEasy),
    ComputerHard(players::optimal::ComputerHard),
}

impl UpdateGameState for Players {
    fn update_game_state(&mut self, state: &guess_game::Turn) {
        match self {
            Players::Human(human) => human.update_game_state(state),
            Players::ComputerEasy(computer) => computer.update_game_state(state),
            Players::ComputerHard(computer) => computer.update_game_state(state),
        }
    }
}

pub struct GuessingGame {
    board: board::GuessingGameBoard,
    players: u8,
    last_player_move: u8,
    player_props: Vec<Players>, // All players: Human + AI
}

impl GuessingGame {
    /// Returns a new Game Object. This object is passed on to the core
    /// engine who simulates the game play
    ///
    ///
    /// - **players** : No of players to play this game
    pub fn new(players: u8, humans: u8, level: Level) -> GuessingGame {
        let mut player_props: Vec<Players> = Vec::new();

        for i in 1..players + 1 {
            if i <= humans {
                println!("Adding Human player with id: {}", i);
                let name = format!("Human {}", i.to_string());
                player_props.push(Players::Human(players::human::HumanPlayer::new(
                    i as i32, &name,
                )));
            } else {
                println!("Adding bot with id: {}", i);
                let name = format!("Computer {}", i);
                match level {
                    Level::Easy => {
                        let bot = Players::ComputerEasy(players::random::ComputerEasy::new(
                            i as i32, &name,
                        ));
                        player_props.push(bot);
                    }
                    Level::Hard => {
                        player_props.push(Players::ComputerHard(
                            players::optimal::ComputerHard::new(i as i32, &name),
                        ));
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
}

impl GuessingGame {
    fn get_player(&self, idx: u8) -> &Players {
        &self.player_props[idx as usize - 1]
    }

    fn simulate(&mut self, turn: String) {
        if self.board.is_valid(&turn) {
            let board_response = self.board.update(&turn);
            let state = guess_game::Turn::GuessingGame(GuessingGameState {
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

use crate::guess_game;

impl guess_game::Start for GuessingGame {
    /// Initialize the state. Sets the board state so that game can be played
    fn init(&mut self) {
        self.board.init();
    }
}

impl guess_game::Update for GuessingGame {
    /// Player plays the next move. Board is updated after every move.
    /// Every move is just a command line read operation
    fn update(&mut self) {
        println!(
            "Player {}'s turn",
            (self.last_player_move % self.players) + 1
        );

        // Returns the Players enum object
        let turn = self.get_player((self.last_player_move % self.players) + 1);

        // Need to do a pattern matching to get the underlying player
        match turn {
            Players::Human(human_player) => {
                let turn = human_player.play();
                println!("Player {} played: {}", human_player.name(), turn);
                self.simulate(turn);
            }
            Players::ComputerEasy(computer) => {
                let turn = computer.play();
                println!("Player {} played: {}", computer.name(), turn);
                self.simulate(turn);
            }
            Players::ComputerHard(computer) => {
                let turn = computer.play();
                println!("Player {} played: {}", computer.name(), turn);
                self.simulate(turn);
            }
            _ => {
                let turn = String::from("Forfeit");
                self.simulate(turn);
            }
        }
    }
}

impl guess_game::Terminate for GuessingGame {
    /// Returns true if the game is over. This game will never result in a tie.
    /// So, skipping the case of tie, in a custom game, you can also add a
    /// handler for tie.
    fn can_terminate(&self) -> bool {
        self.board.terminate()
    }

    fn handle_terminate(&self) {
        println!("Good game, well played. See ya later. Goodbye");
    }
}
