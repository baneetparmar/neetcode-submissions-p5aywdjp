use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

#[derive(Default)]
struct MedianFinder {
    left: BinaryHeap<i32>,
    right: BinaryHeap<Reverse<i32>>,
}

impl MedianFinder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_num(&mut self, num: i32) {
        if let Some(&top) = self.left.peek() {
            if num <= top {
                self.left.push(num);
            } else {
                self.right.push(Reverse(num));
            }
        } else {
            self.left.push(num);
        }
        self.balance();
    }

    pub fn find_median(&self) -> f64 {
        if (self.left.len() + self.right.len()) % 2 == 0 {
            let l = *self.left.peek().unwrap();
            let Reverse(r) = *self.right.peek().unwrap();
            (l as f64 + r as f64) / 2.0
        } else {
            *self.left.peek().unwrap() as f64
        }
    }
    fn balance(&mut self) {
        if self.left.len() > self.right.len() + 1 {
            if let Some(val) = self.left.pop() {
                self.right.push(Reverse(val));
            }
        } else if self.right.len() > self.left.len() {
            if let Some(Reverse(val)) = self.right.pop() {
                self.left.push(val);
            }
        }
    }
}
