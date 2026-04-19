impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtrack(nums: &[i32], visited: &mut Vec<bool>, cur: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
            if cur.len() == nums.len() {
                res.push(cur.clone());
                return;
            }
            for i in 0..nums.len() {
                if visited[i] { continue; }
                visited[i] = true;
                cur.push(nums[i]);
                backtrack(nums, visited, cur, res);
                cur.pop();
                visited[i] = false;
            }
        }

        let mut result = vec![];
        let mut current = vec![];
        let mut visited = vec![false; nums.len()];
        backtrack(&nums, &mut visited, &mut current, &mut result);
        result
    }
}