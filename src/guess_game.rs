pub mod game;
pub mod players;

mod board;

pub trait Start {
    fn init(&mut self);
}

pub trait Update {
    fn update(&mut self) {}
}

pub trait Terminate {
    fn can_terminate(&self) -> bool;

    fn handle_terminate(&self);
}
