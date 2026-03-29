impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (a, b) = if nums1.len() <= nums2.len() {
            (&nums1, &nums2)
        } else {
            (&nums2, &nums1)
        };

        let total = a.len() + b.len();
        let half = (total + 1) / 2;

        let (mut l, mut r) = (0usize, a.len());
        loop {
            let i = (l + r) / 2;
            let j = half - i;

            let a_left = if i > 0 { a[i - 1] as f64 } else { f64::NEG_INFINITY };
            let a_right = if i < a.len() { a[i] as f64 } else { f64::INFINITY };
            let b_left = if j > 0 { b[j - 1] as f64 } else { f64::NEG_INFINITY };
            let b_right = if j < b.len() { b[j] as f64 } else { f64::INFINITY };

            if a_left <= b_right && b_left <= a_right {
                if total % 2 != 0 {
                    return a_left.max(b_left);
                }
                return (a_left.max(b_left) + a_right.min(b_right)) / 2.0;
            } else if a_left > b_right {
                r = i - 1;
            } else {
                l = i + 1;
            }
        }
    }
}