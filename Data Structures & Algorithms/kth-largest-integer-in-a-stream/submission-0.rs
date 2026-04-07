use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    heap: BinaryHeap<Reverse<i32>>,
    k: usize,
}

impl KthLargest {
    pub fn new(k: i32, nums: Vec<i32>) -> Self {
        let k = k as usize;
        let mut heap: BinaryHeap<Reverse<i32>> = nums.into_iter().map(Reverse).collect();

        while heap.len() > k {
            heap.pop();
        }

        Self { heap, k }
    }

    pub fn add(&mut self, val: i32) -> i32 {
        if self.heap.len() < self.k {
            self.heap.push(Reverse(val));
        } else if val > self.heap.peek().unwrap().0 {
            self.heap.pop();
            self.heap.push(Reverse(val));
        }
        // . works because Reverse is tuple struct
        self.heap.peek().unwrap().0
    }
}