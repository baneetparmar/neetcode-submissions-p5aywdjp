impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut l = m - 1;
        let mut r = n - 1;
        let mut pos = (m + n - 1) as usize;
        while l >=0 && r >= 0 {
            if nums1[l as usize ] >= nums2[r as usize] {
                nums1[pos] = nums1[l as usize];
                l -= 1;
            } else {
                nums1[pos] = nums2[r as usize];
                r -= 1;
            }
            pos -= 1
        }
        while l >= 0 {
            nums1[pos] = nums1[l as usize];
            pos -= 1;
            l -= 1;
        }
        while r >= 0 {
            nums1[pos] = nums2[r as usize];
            pos -= 1;
            r -= 1;
        }
    }
}
