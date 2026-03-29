impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() { return 0 }

        let mut pos = 0;
        
        for i in 1..nums.len(){
            if nums[i] != nums[pos] {
                pos += 1;
                nums[pos] = nums[i];
            }
        }
        (pos + 1) as i32
    }
}
