use crate::traits;

pub enum Level {
    Easy,
    Hard,
}

type Bots<Turn, Response> = Vec<traits::player_traits::PlayerBox<Turn, Response>>;

pub struct Game<Board, Turn, Response> {
    board: Board,
    players: Bots<Turn, Response>,
    last_player_move: u8,
}

impl<Board: traits::board_traits::New<Board>, Turn, Response> Game<Board, Turn, Response> {
    pub fn new(bots: Bots<Turn, Response>) -> Game<Board, Turn, Response> {
        Game {
            board: Board::new(),
            last_player_move: bots.len() as u8,
            players: bots,
        }
    }
}

impl<Board: traits::board_traits::Update<Turn, Response>, Turn, Response>
    Game<Board, Turn, Response>
{
    fn get_player(&self, idx: u8) -> &traits::player_traits::PlayerBox<Turn, Response> {
        &self.players[idx as usize - 1]
    }

    /// Checks if the move is valid. If invalid, asks the player to move again,
    /// otherwise returns the result and forwards it to the handlers of all players.
    fn simulate(&mut self, turn: Turn, curr_player: u8) {
        if self.board.is_valid(&turn) {
            let board_response: Response = self.board.update(&turn, curr_player);

            for player in self.players.iter_mut() {
                player.update_game_state(&board_response);
            }
            self.last_player_move = curr_player;
        }
    }
}

impl<Board: traits::game_traits::Start, Turn, Response> traits::game_traits::Start
    for Game<Board, Turn, Response>
{
    fn init(&mut self) {
        self.board.init();
    }
}

impl<Board: traits::board_traits::Update<Turn, Response>, Turn, Response>
    traits::game_traits::Update for Game<Board, Turn, Response>
{
    fn update(&mut self) {
        let num_players = self.players.len() as u8;
        let curr_player = (self.last_player_move) % num_players + 1;
        let player_props = self.get_player(curr_player);
        println!("Player {}'s turn", (*player_props).name());

        let move_played = (*player_props).play();
        self.simulate(move_played, curr_player);
    }
}

impl<Board: traits::game_traits::Terminate, Turn, Response> traits::game_traits::Terminate
    for Game<Board, Turn, Response>
{
    fn can_terminate(&self) -> bool {
        self.board.can_terminate()
    }

    fn handle_terminate(&self) {
        self.board.handle_terminate()
    }
}
