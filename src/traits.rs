pub mod game_traits {
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
}

pub mod player_traits {
    use crate::guess_game;

    pub trait Play {
        fn play(&self) -> String;
    }

    pub enum Turn {
        GuessingGame(guess_game::game::GuessingGameState),
    }

    pub trait UpdateGameState {
        fn update_game_state(&mut self, _turn: &Turn) {}
    }

    pub trait Name {
        fn name(&self) -> String;
    }

    pub trait Player: Play + UpdateGameState + Name {}
}
