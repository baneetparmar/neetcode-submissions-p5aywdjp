impl Solution {
    pub fn subsets_with_dup(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut result = vec![];
        let mut current = vec![];

        fn backtrack(nums: &[i32], start: usize, cur: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            res.push(cur.clone());
            for i in start..nums.len() {
                if i > start && nums[i] == nums[i - 1] { continue; }
                cur.push(nums[i]);
                backtrack(nums, i + 1, cur, res);
                cur.pop();
            }
        }

        backtrack(&nums, 0, &mut current, &mut result);
        result
    }
}