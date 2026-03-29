struct MyCircularQueue {
    capacity:i32,
    size:i32,
    queue:Vec<i32>,
    head:Option<usize>,
    tail:Option<usize>,
}

impl MyCircularQueue {
    pub fn new(k: i32) -> Self {
        Self {
            size:0,
            capacity:k,
            queue:vec![ 0 ;k as usize],
            head:None,
            tail:None,
        }
    }

    pub fn en_queue(&mut self, value: i32) -> bool {
        if self.size < self.capacity {
            match self.tail {
                None => {
                    self.queue[0] = value;
                    self.head = Some(0);
                    self.tail = Some(0);
                },
                Some(t) => {
                    let new_tail = (t + 1) % (self.capacity as usize);
                    self.queue[new_tail] = value;
                    self.tail = Some(new_tail);
                }
            }
            self.size += 1;
            return true
        }
        false
    }

    pub fn de_queue(&mut self) -> bool {
        if self.size > 0 {
            self.size -= 1;
            if self.size == 0 {
                self.head = None;
                self.tail = None;
            } else {
                let new_head = (self.head.unwrap() + 1) % (self.capacity as usize);
                self.head = Some(new_head);
            }
            true
        } else {
            false
        }
    }

    pub fn front(&self) -> i32 {
        match self.head {
            None => -1,
            Some(i) => self.queue[i],
        }
    }
    
    pub fn rear(&self) -> i32 {
        match self.tail {
            None => -1,
            Some(i) => self.queue[i],
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
    
    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }
}

