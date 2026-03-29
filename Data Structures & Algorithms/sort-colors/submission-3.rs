impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut colors = vec![0;3];
        // count 
        for num in nums.iter(){
            colors[*num as usize] += 1
        }
        // fill
        let mut index = 0;
        for (color,freq) in colors.iter().enumerate(){
            for _ in 0..*freq{
                nums[index] = color as i32;
                index += 1;
            }
        }
    }
}
