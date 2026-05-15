use std::collections::{VecDeque, HashSet};

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let dead: HashSet<String> = deadends.into_iter().collect();
        let start = "0000".to_string();
        if dead.contains(&start) { return -1; }
        if start == target { return 0; }

        let mut visited: HashSet<String> = HashSet::new();
        let mut q = VecDeque::new();
        visited.insert(start.clone());
        q.push_back((start, 0));

        while let Some((state, turns)) = q.pop_front() {
            let digits: Vec<u8> = state.bytes().map(|b| b - b'0').collect();
            for i in 0..4 {
                for delta in [1u8, 9u8] {
                    let mut next = digits.clone();
                    next[i] = (next[i] + delta) % 10;
                    let ns: String = next.iter().map(|d| (b'0' + d) as char).collect();
                    if ns == target { return turns + 1; }
                    if !dead.contains(&ns) && !visited.contains(&ns) {
                        visited.insert(ns.clone());
                        q.push_back((ns, turns + 1));
                    }
                }
            }
        }
        -1
    }
}