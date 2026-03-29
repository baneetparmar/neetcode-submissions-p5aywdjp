impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let k = k as usize;

        // our windows of size can have a starting index between 0 and n - k;
        let mut left = 0;
        let mut right = arr.len() - k;

        while left < right {
            // lets search for the optimal window from mid
            // because we have to start from somewhere so why not center of all the windows!?.
            let mid = left + (right - left)/2;

            // if left edge of the window is closer to x
            // so only by towards left can we hope to find better windows
            // similarly if right edge if closer
            // we can only find better windows if we move to right
            if ( x - arr[mid]) <= (arr[mid + k] - x) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        arr[left..left+k].to_vec()
    }
}
