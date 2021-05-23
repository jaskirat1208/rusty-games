pub mod game;
pub mod players;

mod board;

pub trait Start {
    fn init(&mut self);
}

pub trait Update {
    fn update(&mut self);
}

pub trait Terminate {
    fn can_terminate(&self) -> bool;

    fn handle_terminate(&self);
}

pub trait Play {
    fn play(&self) -> String;
}

pub enum Turn {
    GuessingGame(game::GuessingGameState),
}

pub trait UpdateGameState {
    fn update_game_state(&mut self, _turn: &Turn) {}
}

pub trait Name {
    fn name(&self) -> String;
}
