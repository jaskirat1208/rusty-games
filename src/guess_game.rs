
use rand::Rng;
use std::cmp::Ordering;
use std::io;


pub struct GuessingGame {
    m_terminate: bool,
    m_target: i32
}


pub fn read() -> i32 {
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter an integer 0 to 100");
            return -1;
        },
    };

    return guess;
}

impl GuessingGame {
    pub fn new() -> GuessingGame {
        let mut game = GuessingGame {
            m_terminate: false, 
            m_target: 0
        };
        game.init();
        return game;
    }
}

impl GuessingGame {
    /**
     * Initialize the state of the game
     */
    pub fn init(&mut self) {
        self.m_target = rand::thread_rng().gen_range(1..100);
    }

    /**
     * Update the state of the game
     */
    pub fn update(&mut self) {
        let guess = read();

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

impl GuessingGame {
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
