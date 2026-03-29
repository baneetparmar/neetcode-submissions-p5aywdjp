struct MyHashMap {
    imap:Vec<Option<i32>>,
}

impl MyHashMap {
    pub fn new() -> Self {
        let imap = vec![None;10_000_001];
        Self { imap }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        let k = key as usize;
        self.imap[k] = Some(value);
    }

    pub fn get(&self, key: i32) -> i32 {
        let k = key as usize;
        if let Some(val) = self.imap[k]{
            val
        } else {
            -1
        }
    }

    pub fn remove(&mut self, key: i32) {
        let k = key as usize;
        self.imap[k] = None;
    }
}
