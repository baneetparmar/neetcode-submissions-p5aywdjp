impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort();
        let mut result = vec![];
        let mut current = vec![];
        let mut visited = vec![false; nums.len()];

        fn backtrack(nums: &[i32], visited: &mut Vec<bool>, cur: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            if cur.len() == nums.len() {
                res.push(cur.clone());
                return;
            }
            for i in 0..nums.len() {
                if visited[i] { continue; }
                if i > 0 && nums[i] == nums[i - 1] && !visited[i - 1] { continue; } // skip dup siblings
                visited[i] = true;
                cur.push(nums[i]);
                backtrack(nums, visited, cur, res);
                cur.pop();
                visited[i] = false;
            }
        }

        backtrack(&nums, &mut visited, &mut current, &mut result);
        result
    }
}