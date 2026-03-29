impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;
        let left_boundary = 0;
        let right_boundary = arr.len() - 1;

        let center_for_window = (left_boundary..=right_boundary)
                                    .min_by_key(|&index| ((arr[index] - x).abs(),arr[index]))
                                    .unwrap();

        let mut left = center_for_window;
        let mut right = center_for_window;
        while (right - left + 1) < k {
            if left == left_boundary {
                right += 1;
            } else if right  == right_boundary {
                left -= 1;
            }else if (x - arr[right + 1]).abs() < (x - arr[left - 1]).abs(){
                right += 1;
            } else {
                left -= 1;
            }
        }
        Vec::from(&arr[left..=right])
    }
}
