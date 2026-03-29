impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            let mid = left.midpoint(right);

            if nums[mid] == target {
                return mid as i32
            } else if nums[mid] < nums[right] {
                if target > nums[mid] && target <= nums[right]{
                    left = mid + 1;
                } else {
                    right = mid;
                }
            } else {
                if target < nums[mid] && target >= nums[left] {
                    right = mid;
                } else {
                    left = mid + 1;
                }
            }
        }
        -1
    }
}
