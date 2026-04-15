impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn backtrack(index: usize, nums: &Vec<i32>) -> Vec<Vec<i32>> {
            if index == nums.len() {
                return vec![vec![]];
            }

            let rest = backtrack(index + 1, nums);
            let mut res = rest.clone();

            for mut subset in rest {
                subset.push(nums[index]);
                res.push(subset.clone());
            }
            res
        }

        backtrack(0, &nums)
    }
}
