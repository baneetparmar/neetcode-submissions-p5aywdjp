struct MinStack {
    stack:Vec<i32>,
    minimum:Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        Self{
            stack: vec![],
            minimum: vec![],
        }
    }

    pub fn push(&mut self, val: i32) {
        self.stack.push(val);
        if self.minimum.is_empty() ||  val <= *self.minimum.last().unwrap() {
            self.minimum.push(val);
        }
    }

    pub fn pop(&mut self) {
        if let Some(val) = self.stack.pop() {
            if Some(&val) == self.minimum.last() {
                self.minimum.pop();
            }
        }
    }

    pub fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }

    pub fn get_min(&self) -> i32 {
        *self.minimum.last().unwrap()
    }
}
