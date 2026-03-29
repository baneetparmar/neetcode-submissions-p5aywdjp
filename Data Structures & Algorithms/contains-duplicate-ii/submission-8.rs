use std::collections::{HashSet, HashMap};

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        if nums.len() < 2 {
            return false
        }
        let k = k as usize;
        let mut window = HashSet::with_capacity(k + 1);
        for i in 0..nums.len() {
            if !window.insert(nums[i]){
                return true;
            }
            if window.len() > k {
                window.remove(&nums[i - k]);
            }
        }
        false
    }
}
