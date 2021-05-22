use std::io;

pub struct HumanPlayer {
    m_name: String,
    _m_id: i32,
}

pub fn read() -> String {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    return guess;
}

impl HumanPlayer {
    pub fn new(id: i32, name: &String) -> HumanPlayer {
        return HumanPlayer {
            m_name: name.to_string(),
            _m_id: id,
        };
    }
}

impl HumanPlayer {
    pub fn play(&self) -> String {
        return read();
    }

    pub fn name(&self) -> &String {
        return &self.m_name;
    }
}
