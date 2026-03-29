use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut curr_sum = 0;
        let mut res = 0;
        let mut prefix_sum_count:HashMap<i32,i32> = HashMap::from([(0,1)]);
        for num in nums.iter(){
            curr_sum += num;
            let req = curr_sum - k;
            res += prefix_sum_count.get(&req).unwrap_or(&0);
            *prefix_sum_count.entry(curr_sum).or_insert(0) += 1;
        }
        res
    }
}
