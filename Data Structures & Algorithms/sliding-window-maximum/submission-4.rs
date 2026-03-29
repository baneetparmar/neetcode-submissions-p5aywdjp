use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;

        let mut res:Vec<i32> = vec![];
        let mut left = 0;
        let mut right = 0;
        let mut dq = VecDeque::<usize>::new();

        for right in 0..nums.len() {
            while let Some(&back) = dq.back() {
                if nums[back] < nums[right] {
                    dq.pop_back();
                } else { break }
            }
            dq.push_back(right);

            if right >= k - 1 {
                if let Some(&front) = dq.front(){
                    res.push(nums[front]);
                    if front == left {
                        dq.pop_front();
                        }
                };
                left += 1;
            }
        }
        res
    }
}
