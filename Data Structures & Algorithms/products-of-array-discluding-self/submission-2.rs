impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut output = vec![1; n];

        // forward pass — output[i] holds product of everything to the left
        let mut left = 1;
        for i in 0..n {
            output[i] = left;
            left *= nums[i];
        }

        // backward pass — multiply by product of everything to the right
        let mut right = 1;
        for i in (0..n).rev() {
            output[i] *= right;
            right *= nums[i];
        }

        output
    }
}