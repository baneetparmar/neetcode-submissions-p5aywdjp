use std::collections::{HashSet, HashMap};

impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        if nums.len() < 2 {
            return false
        }

        let mut last_seen = HashMap::new();
        for (index,number) in nums.iter().enumerate() {
            if let Some(&last_know_index) = last_seen.get(number) {
                if (index - last_know_index) <= k as usize {
                    return true
                }
            }
            last_seen.insert(number,index);
        }
        false
    }
}
