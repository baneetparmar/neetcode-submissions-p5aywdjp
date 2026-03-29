// The guess API is already defined for you.
// fn guess(num: i64) -> i32:
//     -1 if num is higher than the picked number
//      1 if num is lower than the picked number
//      0 if num is equal to the picked number

impl Solution {
    pub unsafe fn guess_number(n: i64) -> i64 {
        let mut left = 0;
        let mut right = n + 1;
        loop {
            let mid = left + (right - left)/2;
            match guess(mid) {
                1 =>  left = mid + 1,
                -1 =>  right = mid - 1,
                0 =>  return mid as i64,
                _ => unreachable!(),
            }
        }
    }
}
