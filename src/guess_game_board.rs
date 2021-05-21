
use rand::Rng;
use std::cmp::Ordering;


pub struct GuessingGameBoard {
    m_terminate: bool,
    m_target: i32
}




impl GuessingGameBoard {
    pub fn new() -> GuessingGameBoard {
        let game = GuessingGameBoard {
            m_terminate: false, 
            m_target: 0
        };
        return game;
    }
}

impl GuessingGameBoard {
    /**
     * Initialize the state of the game
     */
    pub fn init(&mut self) {
        self.m_target = rand::thread_rng().gen_range(1..100);
    }

    /**
     * Update the state of the game
     */
    pub fn update(&mut self, guess: i32) {

        match guess.cmp(&self.m_target) {
            Ordering::Equal => self.handle_equal(),
            Ordering::Less => self.handle_less(),
            Ordering::Greater => self.handle_greater()
        }
    }

    /**
     * Condition for termination of the event loop
     */
    pub fn terminate(&self) -> bool {
        return self.m_terminate;
    }
}

impl GuessingGameBoard {
    fn handle_less(&self) {
        println!("Too small");
    }
    
    fn handle_greater(&self) {
        println!("Too big");
    }
    
    fn handle_equal(&mut self) {
        println!("Victory: You guessed it correct");
        self.m_terminate = true;
    }
}
