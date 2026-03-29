impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        let size = nums.len();
        let mut res = Vec::from(nums.clone());
        for i in 0..size {
            res.push(nums[i]);
        }
        return res
    }
}
