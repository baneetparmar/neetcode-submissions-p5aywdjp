impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let mut maximum_water = 0;
        let mut l = 0;
        let mut r = heights.len() - 1;
        while l < r {
            let water_stored = heights[l].min(heights[r]) *  (r - l) as i32;
            maximum_water = maximum_water.max(water_stored);
            if heights[l] <= heights[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }
        maximum_water
    }
}
