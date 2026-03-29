impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut left = 1;
        let mut right = *piles.iter().max().unwrap();
        let mut eating_rate = right;

        while left <= right {
            let rate = left + (right - left) / 2;

            let time_taken: i64 = piles.iter()
                .map(|&p| (p as i64 + rate as i64 - 1) / rate as i64)
                .sum();

            if time_taken <= h as i64 {
                eating_rate = rate;  // valid — try slower
                right = rate - 1;
            } else {
                left = rate + 1;     // too slow — try faster
            }
        }

        eating_rate
    }
}