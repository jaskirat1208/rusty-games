use puzzle_games::engine;
use puzzle_games::guess_game;
use puzzle_games::traits;

use std::cmp;

type PlayerBox = Box<dyn traits::player_traits::Player>;

struct MyCustomBot {
    name: String,
    start: i32,
    end: i32,
}

impl traits::player_traits::Play for MyCustomBot {
    fn play(&self) -> String {
        if self.end - self.start < 15 {
            return (self.start + (self.end - self.start) / 2).to_string();
        }
        return self.start.to_string();
    }
}

impl traits::player_traits::Name for MyCustomBot {
    fn name(&self) -> String {
        return self.name.to_string();
    }
}

impl traits::player_traits::UpdateGameState for MyCustomBot {
    fn update_game_state(&mut self, turn: &traits::player_traits::Turn) {
        match turn {
            traits::player_traits::Turn::GuessingGame(state) => {
                match state.board_response.result {
                    cmp::Ordering::Less => {
                        self.start = cmp::max(self.start, state.board_response.move_played);
                    }
                    cmp::Ordering::Greater => {
                        self.end = cmp::min(self.end, state.board_response.move_played);
                    }
                    cmp::Ordering::Equal => {}
                };
            }
        }
    }
}

impl traits::player_traits::Player for MyCustomBot {}

fn main() {
    let mut bots: Vec<PlayerBox> = Vec::new();
    let easy_bot = guess_game::players::random::ComputerEasy::new(0, "Mablo".to_string());
    let hard_bot = guess_game::players::optimal::ComputerHard::new(1, "Superman".to_string());
    bots.push(Box::new(easy_bot));
    bots.push(Box::new(hard_bot));

    let custom = MyCustomBot {
        name: "Ultraman".to_string(),
        start: 1,
        end: 100,
    };
    bots.push(Box::new(custom));

    let mut game = guess_game::game::GuessingGame::new_w_custom_bots(bots);

    engine::start(&mut game);
}