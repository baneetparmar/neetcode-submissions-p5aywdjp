use std::collections::HashMap;

#[derive(Default)]
struct FreqStack {
    count:HashMap<i32,i32>,
    stack:HashMap<i32,Vec<i32>>,
    max_cnt: i32,
}

impl FreqStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, val: i32) {
        *self.count.entry(val).or_insert(0) += 1;
        let &cnt = self.count.get(&val).unwrap();
        if cnt > self.max_cnt {
            self.max_cnt = cnt;
            self.stack.entry(cnt).or_insert(Vec::new());
        }
        self.stack.get_mut(&cnt).unwrap().push(val);

    }

    pub fn pop(&mut self) -> i32 {
        let res = self.stack.get_mut(&self.max_cnt).unwrap().pop().unwrap();
        *self.count.get_mut(&res).unwrap() -= 1;
        if self.stack[&self.max_cnt].is_empty() {
            self.max_cnt -= 1;
        }
        res
    }
}
