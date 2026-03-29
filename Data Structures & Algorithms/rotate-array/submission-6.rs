impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = (k as usize) % nums.len();
        if k == 0 {
            return 
        }
        let mut nums = nums;
        nums.reverse();
        let mut l = 0;
        let mut r = k - 1;
        while l < r {
            nums.swap(l,r);
            l += 1;
            r -= 1;
        }
        let mut l2 = k as usize;
        let mut r2 = nums.len() - 1;
        while l2 < r2 {
            nums.swap(l2,r2);
            l2 += 1;
            r2 -= 1;
        }
    }
}
