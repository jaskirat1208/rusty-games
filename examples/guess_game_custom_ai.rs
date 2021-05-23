use puzzle_games::engine;
use puzzle_games::game;
use puzzle_games::guess_game;
use puzzle_games::traits;

use std::cmp;

type PlayerBox = traits::player_traits::PlayerBox<String, guess_game::board::BoardResponse>;

struct MyCustomBot {
    name: String,
    start: i32,
    end: i32,
}

impl traits::player_traits::Play<String> for MyCustomBot {
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

impl traits::player_traits::UpdateGameState<guess_game::board::BoardResponse> for MyCustomBot {
    fn update_game_state(&mut self, response: &guess_game::board::BoardResponse) {
        match response.result {
            cmp::Ordering::Less => {
                self.start = cmp::max(self.start, response.move_played);
            }
            cmp::Ordering::Greater => {
                self.end = cmp::min(self.end, response.move_played);
            }
            cmp::Ordering::Equal => {}
        };
    }
}

impl traits::player_traits::Player<String, guess_game::board::BoardResponse> for MyCustomBot {}

fn main() {
    let mut bots: Vec<PlayerBox> = Vec::new();
    let easy_bot = guess_game::players::optimal::ComputerHard::new(0, "Mablo".to_string());
    let hard_bot = guess_game::players::optimal::ComputerHard::new(1, "Superman".to_string());
    bots.push(Box::new(easy_bot));
    bots.push(Box::new(hard_bot));

    let custom = MyCustomBot {
        name: "Ultraman".to_string(),
        start: 1,
        end: 100,
    };
    bots.push(Box::new(custom));

    let mut game = game::Game::<
        guess_game::board::GuessingGameBoard,
        String,
        guess_game::board::BoardResponse,
    >::new(bots);

    engine::start(&mut game);
}
