impl Solution {
    pub fn combination_sum2(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn backtrack(nums: &[i32], remaining: i32, start: usize, cur: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            if remaining == 0 {
                res.push(cur.clone());
                return;
            }
            for i in start..nums.len() {
                if nums[i] > remaining { break; }
                if i > start && nums[i] == nums[i-1] { continue; }
                cur.push(nums[i]);
                backtrack(nums, remaining - nums[i], i + 1, cur, res);
                cur.pop();
            }
        }

        let mut nums = nums;
        nums.sort();
        let mut current = vec![];
        let mut result = vec![];
        backtrack(&nums, target, 0, &mut current, &mut result);
        result
    }
}