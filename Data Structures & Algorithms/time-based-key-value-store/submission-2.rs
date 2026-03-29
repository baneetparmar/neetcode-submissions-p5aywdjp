use std::collections::HashMap;

#[derive(Default)]
struct TimeMap {
    map:HashMap<String,Vec<(String,i32)>>,
}

impl TimeMap {
    fn new() -> Self {
        Default::default()
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.map.entry(key).or_default().push((value,timestamp))
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        match self.map.get(&key) {
            None => String::new(),
            Some(values) => {
                let p = values.partition_point(|& (_, t) | t <= timestamp);
                if p == 0 {
                    String::new()
                } else {
                    values[p-1].0.clone()
                }
            }
        }
    }
}
