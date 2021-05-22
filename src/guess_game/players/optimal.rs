pub struct ComputerHard {
    m_name: String,
    _m_id: i32  
}

impl ComputerHard {
    pub fn new(id: i32, name: &String) -> ComputerHard {
        return ComputerHard {
            m_name: name.to_lowercase(),
            _m_id: id
        };
    }

    pub fn name(&self) -> &String {
        return &self.m_name;
    }
}