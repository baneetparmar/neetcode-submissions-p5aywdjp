struct MyHashSet {
    set: Vec<bool>,
}

impl MyHashSet {
    pub fn new() -> Self {
        Self{ set:vec![false;10_000_001] }
    }

    pub fn add(&mut self, key: i32) {
        self.set[key as usize] = true;   
    }

    pub fn remove(&mut self, key: i32) {
        self.set[key as usize] = false;
    }

    pub fn contains(&self, key: i32) -> bool {
        self.set[key as usize]
    }
}
