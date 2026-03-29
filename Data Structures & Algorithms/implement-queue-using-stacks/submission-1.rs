struct MyQueue {
    queue: Vec<i32>,
    tmp: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        MyQueue {
            queue: Vec::new(),
            tmp: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.queue.push(x);
    }

    fn pop(&mut self) -> i32 {
        while let Some(val) = self.queue.pop() {
            self.tmp.push(val);
        }
        let first = self.tmp.pop().unwrap();
        while let Some(val) = self.tmp.pop() {
            self.queue.push(val);
        }
        first
    }

    fn peek(&self) -> i32 {
        self.queue[0]
    }

    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}