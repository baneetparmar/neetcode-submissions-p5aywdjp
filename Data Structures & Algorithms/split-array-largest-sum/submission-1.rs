impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let mut l = *nums.iter().max().unwrap();
        let mut r = nums.iter().sum::<i32>() + 1;
        while l < r {
            let m = l + (r - l)/2;
            let mut sum = 0;
            let mut splits = 0;
            for &n in nums.iter() {
                if sum + n > m {
                    splits += 1;
                    sum = n;
                } else {
                    sum += n;
                }
            }
            if sum > 0 {
                splits += 1;
            }
            if splits > k {
                l = m + 1;
            } else {
                r = m;
            }
        }
    r as i32
    }
}
