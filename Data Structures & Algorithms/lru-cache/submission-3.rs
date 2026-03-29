use std::collections::HashMap;
use std::ptr;

struct Node {
    key: i32,
    val: i32,
    prev: *mut Node,
    next: *mut Node,
}

impl Node {
    fn new(key: i32, val: i32) -> *mut Node {
        Box::into_raw(Box::new(Node {
            key,
            val,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }))
    }
}

struct LRUCache {
    capacity: usize,
    map: HashMap<i32, *mut Node>,
    // sentinel nodes — never removed, simplify edge cases
    head: *mut Node,  // most recent
    tail: *mut Node,  // least recent
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        let head = Node::new(0, 0);
        let tail = Node::new(0, 0);
        // wire sentinels together
        unsafe {
            (*head).next = tail;
            (*tail).prev = head;
        }
        Self {
            capacity: capacity as usize,
            map: HashMap::new(),
            head,
            tail,
        }
    }

    // remove a node from wherever it is in the list
    unsafe fn remove(node: *mut Node) {
        let prev = (*node).prev;
        let next = (*node).next;
        (*prev).next = next;
        (*next).prev = prev;
    }

    // insert a node right after head (most recent position)
    unsafe fn insert_front(&mut self, node: *mut Node) {
        let after_head = (*self.head).next;
        (*self.head).next = node;
        (*node).prev = self.head;
        (*node).next = after_head;
        (*after_head).prev = node;
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(&node) = self.map.get(&key) {
            unsafe {
                // move to front — most recently used
                Self::remove(node);
                self.insert_front(node);
                (*node).val
            }
        } else {
            -1
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        unsafe {
            if let Some(&node) = self.map.get(&key) {
                // update existing — move to front
                (*node).val = value;
                Self::remove(node);
                self.insert_front(node);
            } else {
                // insert new node at front
                let node = Node::new(key, value);
                self.map.insert(key, node);
                self.insert_front(node);

                if self.map.len() > self.capacity {
                    // evict LRU — node just before tail sentinel
                    let lru = (*self.tail).prev;
                    Self::remove(lru);
                    let lru_box = Box::from_raw(lru);
                    self.map.remove(&lru_box.key);
                    // lru_box dropped here — memory freed
                }
            }
        }
    }
}

impl Drop for LRUCache {
    fn drop(&mut self) {
        unsafe {
            // walk and free all real nodes
            let mut cur = (*self.head).next;
            while cur != self.tail {
                let next = (*cur).next;
                drop(Box::from_raw(cur));
                cur = next;
            }
            // free sentinels
            drop(Box::from_raw(self.head));
            drop(Box::from_raw(self.tail));
        }
    }
}