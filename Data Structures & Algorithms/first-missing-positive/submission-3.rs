impl Solution {
    pub fn first_missing_positive(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut i = 0;

        while i < n {
            let val = nums[i];

            if val > 0 && (val as usize) <= n {
                let target_index = (val - 1) as usize;

                if nums[target_index] != val {
                    nums.swap(i, target_index);
                } else {
                    i += 1;
                }
            } else {
                i += 1;
            }
        }

        for i in 0..n {
            if nums[i] != (i as i32 + 1) {
                return i as i32 + 1;
            }
        }

        n as i32 + 1
    }
}