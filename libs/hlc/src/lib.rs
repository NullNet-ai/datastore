use std::time::SystemTime;

pub struct Timestamp {
    logical: u64,
    physical: u64,
    node_id: String,
}

impl Timestamp {
    pub fn new(logical: u64, physical: u64, node_id: String) -> Self {
        Timestamp {
            logical,
            physical,
            node_id,
        }
    }
    pub fn to_string(&self) -> String {
        format!("{}:{}:{}", self.physical, self.logical, self.node_id)
    }

    pub fn parse(str: String) -> Self {
        let parts: Vec<&str> = str.split(':').collect();
        Timestamp {
            logical: parts[0].parse().unwrap(),
            physical: parts[1].parse().unwrap(),
            node_id: parts[2].to_string(),
        }
    }

    pub fn send(&mut self, other: &Timestamp) {
        self.logical = std::cmp::max(self.logical, other.logical) + 1;
        self.physical = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    pub fn recv(&mut self, other: &Timestamp) {
        self.logical = std::cmp::max(self.logical, other.logical) + 1;
        self.physical = std::cmp::max(self.physical, other.physical);
    }
}
