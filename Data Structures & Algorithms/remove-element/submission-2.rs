impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32{
        let mut pos:usize = 0;
        let mut selector:usize = pos;
        while selector < nums.len(){
            if nums[selector] != val{
                nums.swap(pos,selector);
                pos += 1;
            }
            selector += 1
        }
        pos as i32
    }
}
