use std::collections::HashMap;
use std::ptr;

struct Node {
    key: i32,
    val: i32,
    prev: *mut Node,
    next: *mut Node,
}

impl Node {
    pub fn new(key: i32, val: i32) -> *mut Self {
        Box::into_raw(Box::new(Node {
            key,
            val,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }))
    }
}

struct LRUCache {
    head: *mut Node,
    tail: *mut Node,
    size: i32,
}

impl LRUCache {
    fn new() -> Self {
        let head = Node::new(0, 0);
        let tail = Node::new(0, 0);
        unsafe {
            (*head).next = tail;
            (*tail).prev = head;
        }
        Self { head, tail, size: 0 }
    }

    pub unsafe fn remove(node: *mut Node) {
        let p = (*node).prev;
        let n = (*node).next;
        (*p).next = n;
        (*n).prev = p;
    }

    pub unsafe fn insert_from_front(&mut self, node: *mut Node) {
        let after_head = (*self.head).next;
        (*self.head).next = node;
        (*after_head).prev = node;
        (*node).prev = self.head;
        (*node).next = after_head;
    }
}

impl Drop for LRUCache {
    fn drop(&mut self) {
        unsafe {
            let mut cur = (*self.head).next;
            while cur != self.tail {
                let next = (*cur).next;
                drop(Box::from_raw(cur));
                cur = next;
            }
            drop(Box::from_raw(self.head));
            drop(Box::from_raw(self.tail));
        }
    }
}

struct LFUCache {
    minimum_frequency: i32,
    key_value_map: HashMap<i32, *mut Node>,
    key_frequency_map: HashMap<i32, i32>,
    frequency_lru_key: HashMap<i32, LRUCache>,
    size: i32,
    capacity: i32,
}

impl LFUCache {
    pub fn new(capacity: i32) -> Self {
        Self {
            size: 0,
            capacity,
            minimum_frequency: 0,
            key_value_map: HashMap::new(),
            key_frequency_map: HashMap::new(),
            frequency_lru_key: HashMap::new(),
        }
    }

    unsafe fn increment_frequency(&mut self, key: i32, node: *mut Node) {
        let freq = *self.key_frequency_map.get(&key).unwrap();

        // remove from current bucket
        let bucket = self.frequency_lru_key.get_mut(&freq).unwrap();
        LRUCache::remove(node);
        bucket.size -= 1;

        // update minimum_frequency if bucket now empty
        if bucket.size == 0 && self.minimum_frequency == freq {
            self.minimum_frequency += 1;
        }

        // insert into freq+1 bucket
        let new_freq = freq + 1;
        self.key_frequency_map.insert(key, new_freq);
        self.frequency_lru_key
            .entry(new_freq)
            .or_insert_with(LRUCache::new)
            .insert_from_front(node);
        self.frequency_lru_key
            .get_mut(&new_freq)
            .unwrap()
            .size += 1;
    }

    fn remove_lfu(&mut self) {
        let freq = self.minimum_frequency;
        let bucket = self.frequency_lru_key.get_mut(&freq).unwrap();
        unsafe {
            // LRU node is just before tail
            let node_to_del = (*bucket.tail).prev;
            LRUCache::remove(node_to_del);
            bucket.size -= 1;
            let key = (*node_to_del).key;
            self.key_value_map.remove(&key);
            self.key_frequency_map.remove(&key);
            drop(Box::from_raw(node_to_del)); // free memory
        }
        self.size -= 1;
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(&node) = self.key_value_map.get(&key) {
            unsafe {
                let val = (*node).val;
                self.increment_frequency(key, node);
                val
            }
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if self.capacity <= 0 { return; }

        if let Some(&node) = self.key_value_map.get(&key) {
            // key exists — update value and bump frequency
            unsafe {
                (*node).val = value;
                self.increment_frequency(key, node);
            }
        } else {
            // evict if at capacity
            if self.size == self.capacity {
                self.remove_lfu();
            }

            // insert new node
            let node = Node::new(key, value);
            self.size += 1;
            self.minimum_frequency = 1;
            self.key_value_map.insert(key, node);
            self.key_frequency_map.insert(key, 1);

            let bucket = self.frequency_lru_key
                .entry(1)
                .or_insert_with(LRUCache::new);
            bucket.size += 1;
            unsafe { bucket.insert_from_front(node); }
        }
    }
}