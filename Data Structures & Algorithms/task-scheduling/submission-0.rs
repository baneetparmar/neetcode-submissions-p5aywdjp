use std::collections::{BinaryHeap, HashMap, VecDeque};

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut map = HashMap::new();
        for ch in tasks {
            *map.entry(ch).or_insert(0) += 1;
        }

        let mut time = 0;
        let mut deq: VecDeque<(i32, i32)> = VecDeque::new();
        let mut max_heap = BinaryHeap::new();
        for v in map.into_values() {
            max_heap.push(v);
        }
        while !max_heap.is_empty() || !deq.is_empty() {
            while let Some(&(count, ready_at)) = deq.front() {
                if ready_at == time {
                    max_heap.push(count);
                    deq.pop_front();
                } else {
                    break;
                }
            }
            time += 1;
            if let Some(count) = max_heap.pop() {
                if count - 1 > 0 {
                    deq.push_back((count - 1, time + n))
                }
            }
        }
        time
    }
}
