impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            let mid = left + (right - left)/2;

            if nums[mid] == target { return true }

            if nums[left] == nums[mid] && nums[mid] == nums[right] {
                // can't find sorted half.
                left += 1;
                right -= 1;
            } else if nums[mid] <= nums[right] {
                if nums[mid] < target && target <= nums[right] {
                    // move right.
                    left = mid + 1;
                } else {
                    // not in right half. move left.
                    right = mid;
                }
            } else {
                if nums[left] <= target && target < nums[mid] {
                    // move left.
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
        }
        nums[left] == target       
    }
}
