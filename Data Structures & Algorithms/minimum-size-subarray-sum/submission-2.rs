impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut minimum_subarray_length = nums.len() + 1;
        let mut left = 0;
        let mut right = 0;
        let mut current_sum = 0;
        while right < nums.len(){
            current_sum += nums[right];

            while current_sum >= target {
                minimum_subarray_length = minimum_subarray_length.min(right - left + 1);
                current_sum -= nums[left];
                left += 1;
            }
            right += 1;
        }
        if minimum_subarray_length < nums.len() + 1 { minimum_subarray_length as i32 } else { 0i32 }
    }
}
