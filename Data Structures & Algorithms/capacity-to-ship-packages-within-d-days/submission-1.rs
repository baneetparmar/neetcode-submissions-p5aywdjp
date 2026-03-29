impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let  &(mut left) = weights.iter().max().unwrap();
        let mut right = weights.iter().sum::<i32>();
        let mut least_weight_capacity = right;

        while left <= right {
            let mid = left.midpoint(right);
            let mut curr_sum = 0;
            let days_needed = {
                let mut d = 0;
                for &weight in weights.iter() {
                    if curr_sum + weight > mid {
                        curr_sum = weight;
                        d += 1;
                    } else {
                        curr_sum += weight;
                    }
                }
                if curr_sum > 0 {
                    d += 1;
                }
                d
            };

            if days_needed <= days { // valid -> can decrease needed capacity
                least_weight_capacity = mid;
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        least_weight_capacity
    }
}
