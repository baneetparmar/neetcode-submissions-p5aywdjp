impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let x = x as i64;

        let mut left = 0;
        let mut right = x;

        while left <= right {
            let mid = left + (right - left)/2;
            let sq = mid * mid;
            if sq == x {
                return mid as i32;
            } else if sq < x {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        right as i32
    }
}
