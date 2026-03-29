impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> Vec<i32> {
        let (mut candidate1, mut candidate2) = (0, 1);
        let (mut count1, mut count2) = (0i32, 0i32);

        // Phase 1: Find candidates
        for &num in &nums {
            if num == candidate1 {
                count1 += 1;
            } else if num == candidate2 {
                count2 += 1;
            } else if count1 == 0 {
                candidate1 = num;
                count1 = 1;
            } else if count2 == 0 {
                candidate2 = num;
                count2 = 1;
            } else {
                count1 -= 1;
                count2 -= 1;
            }
        }

        // Phase 2: Verify candidates
        count1 = 0;
        count2 = 0;
        for &num in &nums {
            if num == candidate1 {
                count1 += 1;
            } else if num == candidate2 {
                count2 += 1;
            }
        }

        let threshold = (nums.len() / 3) as i32;
        let mut result = Vec::new();
        if count1 > threshold {
            result.push(candidate1);
        }
        if count2 > threshold {
            result.push(candidate2);
        }
        result
    }
}
