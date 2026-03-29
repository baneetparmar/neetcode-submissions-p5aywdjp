use std::collections::HashMap;

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        if nums.len() < 2 {
            return false
        }

        let mut buckets:HashMap<i32,Vec<usize>> = HashMap::new();
        for i in 0..nums.len(){
            buckets.entry(nums[i]).or_insert(vec![]).push(i);
        }

        for values in buckets.values() {
            if values.len() > 1 {
                for window in values.windows(2) {
                    if window[1] - window[0] <= k as usize {
                        return true
                    }
                }
            }
        }
        false
    }
}
