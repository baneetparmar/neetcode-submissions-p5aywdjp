use std::collections::HashMap;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut res = vec![];
        let max_count = nums.len() + 1;

        let mut counter:HashMap<i32,usize> = HashMap::new();

        for num in nums {
            *counter.entry(num).or_insert(0) += 1;
        }

        let mut buckets:Vec<Vec<i32>> = vec![vec![]; max_count];
        for (&num, &count) in counter.iter() {
            buckets[count].push(num);
        }
        for bucket in buckets.iter().rev(){
            for &num in bucket {
                res.push(num);
                if res.len() == k as usize {
                    return res;
                }
            }
        }
        unreachable!("constraint: there always exists a unique solution")
    }
}
