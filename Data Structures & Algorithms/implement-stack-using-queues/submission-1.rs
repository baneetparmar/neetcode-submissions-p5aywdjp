use std::collections::VecDeque;

struct MyStack {
    stack: VecDeque<i32>,
    tmp: VecDeque<i32>,
}

impl MyStack {
    fn new() -> Self {
        MyStack {
            stack: VecDeque::new(),
            tmp: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.stack.push_back(x);
    }

    fn pop(&mut self) -> i32 {
        while !self.stack.is_empty() {
            let ele = self.stack.pop_front().unwrap();
            if self.stack.is_empty() {
                while let Some(val) = self.tmp.pop_front() {
                    self.stack.push_back(val);
                }
                return ele;
            } else {
                self.tmp.push_back(ele);
            }
        }
        unreachable!()
    }

    fn top(&self) -> i32 {
        *self.stack.back().unwrap()
    }

    fn empty(&self) -> bool {
        self.stack.is_empty()
    }
}