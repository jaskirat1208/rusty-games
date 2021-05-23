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

    pub trait Play<PlayerMove> {
        fn play(&self) -> PlayerMove;
    }

    pub trait UpdateGameState<Response> {
        fn update_game_state(&mut self, _turn: &Response) {}
    }

    pub trait Name {
        fn name(&self) -> String;
    }

    pub trait Player<Turn, Response>: Play<Turn> + UpdateGameState<Response> + Name {}

    pub type PlayerBox<PlayerMove, Response> = Box<dyn Player<PlayerMove, Response>>;
}

pub mod board_traits {

    pub trait New<Board> {
        fn new() -> Self;
    }

    pub trait Info {
        fn board_response() {}
    }
    pub trait Update<PlayerMove, Response> {
        fn is_valid(&self, turn: &PlayerMove) -> bool;
        fn update(&mut self, turn: &PlayerMove, curr_player: u8) -> Response;
    }
}
