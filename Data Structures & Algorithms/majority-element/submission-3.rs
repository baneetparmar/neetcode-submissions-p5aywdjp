impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        // ignored: constaint 1<= nums.length <= 50_000
        // if nums.is_empty(){return}

        let mut candidate:i32 = nums[0];
        let mut votes:i32 = 0;
        for num in nums{
            if votes == 0{
                candidate = num;
            }
            if num == candidate{votes += 1;}
             else {votes -= 1;}
        }
        candidate
    }
}
