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
    use crate::total_sum;

    pub trait Play<PlayerMove> {
        fn play(&self) -> PlayerMove;
    }

    pub enum Turn {
        GuessingGame(guess_game::game::GuessingGameState),
        TotalSum(total_sum::board::BoardResponse),
    }

    pub trait UpdateGameState<Response> {
        fn update_game_state(&mut self, _turn: &Response) {}
    }

    pub trait Name {
        fn name(&self) -> String;
    }

    pub trait Player<Turn, Response>: Play<Turn> + UpdateGameState<Response> + Name {}

    pub type PlayerBox = Box<dyn Player<String, Turn>>;

    pub type TotalSumPlayerBox<PlayerMove, Response> = Box<dyn Player<PlayerMove, Response>>;
}

pub mod board_traits {
    pub trait New<Board> {
        fn new() -> Board;
    }

    pub trait Info {
        fn board_response() {}
    }
    pub trait Update<MoveInfo, BoardInfo> {
        fn is_valid(&self, turn: &MoveInfo) -> bool;
        fn update(&mut self, turn: &MoveInfo, curr_player: u8) -> BoardInfo;
    }
}
