use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen:HashMap<i32,usize> = HashMap::new();
        for i in 0..nums.len(){
            if let Some(&j) = seen.get(&(target - nums[i])){
                return vec![j as i32,i as i32];
            }
            seen.insert(nums[i],i);
        }
        unreachable!()
    }
}
