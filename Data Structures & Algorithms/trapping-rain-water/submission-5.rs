impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        
        if n == 0 {
            return 0;
        }
        
        // Build array of max heights to the left of each position
        let mut max_heights_from_left: Vec<i32> = vec![];
        let mut current_max_from_left = 0;
        
        for &current_height in height.iter() {
            // Store the max height seen so far (before current position)
            max_heights_from_left.push(current_max_from_left);
            // Update max for next iteration
            current_max_from_left = current_max_from_left.max(current_height);
        }
        
        // Traverse right to left, tracking max height from right
        let mut trapped_water_total = 0;
        let mut current_max_from_right = 0;
        
        for index in (0..n).rev() {
            // Water level at this position = min(max_left, max_right)
            let water_level = current_max_from_right.min(max_heights_from_left[index]);
            
            // Water trapped = water_level - ground_level
            let water_trapped_at_index = water_level - height[index];
            
            // Only add if positive (ground higher than water level means no trap)
            if water_trapped_at_index > 0 {
                trapped_water_total += water_trapped_at_index;
            }
            
            // Update max height from right for next iteration
            current_max_from_right = current_max_from_right.max(height[index]);
        }
        
        trapped_water_total
    }
}