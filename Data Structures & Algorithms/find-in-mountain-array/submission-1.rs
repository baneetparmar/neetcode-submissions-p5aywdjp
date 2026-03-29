impl Solution {
    pub fn find_in_mountain_array(target: i32, mountain_arr: &dyn MountainArray) -> i32 {
        let n = mountain_arr.length();
        let peak = Self::find_peak(mountain_arr, n);

        if let Some(i) = Self::binary_search_asc(mountain_arr, target, 0, peak) {
            return i;
        }
        if let Some(i) = Self::binary_search_desc(mountain_arr, target, peak + 1, n - 1) {
            return i;
        }
        -1
    }

    fn find_peak(arr: &dyn MountainArray, n: i32) -> i32 {
        let (mut l, mut r) = (0, n - 1);

        while l < r {
            let m = l + (r - l) / 2;
            let mid = arr.get(m);
            let next = arr.get(m + 1);

            if mid < next {
                l = m + 1;
            } else {
                r = m;
            }
        }
        l
    }

    fn binary_search_asc(
        arr: &dyn MountainArray,
        target: i32,
        mut l: i32,
        mut r: i32,
    ) -> Option<i32> {
        while l <= r {
            let m = l + (r - l) / 2;
            let val = arr.get(m);

            if val == target {
                return Some(m);
            } else if val < target {
                l = m + 1;
            } else {
                r = m - 1;
            }
        }
        None
    }

    fn binary_search_desc(
        arr: &dyn MountainArray,
        target: i32,
        mut l: i32,
        mut r: i32,
    ) -> Option<i32> {
        while l <= r {
            let m = l + (r - l) / 2;
            let val = arr.get(m);

            if val == target {
                return Some(m);
            } else if val > target {
                l = m + 1;
            } else {
                r = m - 1;
            }
        }
        None
    }
}