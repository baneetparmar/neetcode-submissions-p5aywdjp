impl Solution {
    pub fn find_kth_largest(mut nums: Vec<i32>, k: i32) -> i32 {
        let k = nums.len() - k as usize;
        nums.sort();
        nums[k]
    }
}