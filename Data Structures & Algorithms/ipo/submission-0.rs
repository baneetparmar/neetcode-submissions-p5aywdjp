use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};

impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut min_heap: BinaryHeap<Reverse<(i32, i32)>> = BinaryHeap::new();
        let mut max_heap: BinaryHeap<i32> = BinaryHeap::new();

        for (cap, prof) in capital.into_iter().zip(profits) {
            min_heap.push(Reverse((cap, prof)));
        }
        let mut pocket_money = w;

        for _ in 0..k {
            while let Some(&Reverse((cash_req, profit))) = min_heap.peek() {
                if cash_req <= pocket_money {
                    max_heap.push(profit);
                    min_heap.pop();
                } else {
                    break;
                }
            }

            if max_heap.is_empty() {
                break;
            }

            if let Some(profit_made) = max_heap.pop() {
                pocket_money += profit_made;
            }
        }
        pocket_money
    }
}
