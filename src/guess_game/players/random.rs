use rand::Rng;

pub struct ComputerEasy {
    m_name: String,
    _m_id: i32,
}

impl ComputerEasy {
    pub fn new(id: i32, name: &String) -> ComputerEasy {
        return ComputerEasy {
            m_name: name.to_string(),
            _m_id: id,
        };
    }
}

impl ComputerEasy {
    pub fn play(&self) -> String {
        return rand::thread_rng().gen_range(1..100).to_string();
    }

    pub fn name(&self) -> &String {
        return &self.m_name;
    }
}
