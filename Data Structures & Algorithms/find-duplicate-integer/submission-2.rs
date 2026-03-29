impl Solution {
pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    let mut slow = 0;
    let mut fast = 0;
    let mut first = true;
    while slow != fast || first {
        first = false;
        slow = nums[slow as usize];
        fast = nums[fast as usize];
        fast = nums[fast as usize];
    }
    let mut slow = 0;
    while slow != fast {
        slow = nums[slow as usize];
        fast = nums[fast as usize];
    }
    slow
}


}
