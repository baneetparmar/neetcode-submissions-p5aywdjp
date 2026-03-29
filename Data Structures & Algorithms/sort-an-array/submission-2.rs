impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();

        // 1. build max heap
        for i in (0..n / 2).rev() {
            Self::heapify(&mut nums, n, i);
        }

        // 2. extract max one by one
        for end in (1..n).rev() {
            nums.swap(0, end);              // move root (max) to end
            Self::heapify(&mut nums, end, 0); // restore heap on reduced array
        }

        nums
    }

    fn heapify(nums: &mut Vec<i32>, n: usize, root: usize) {
        let mut largest = root;
        let left  = 2 * root + 1;
        let right = 2 * root + 2;

        if left  < n && nums[left]  > nums[largest] { largest = left;  }
        if right < n && nums[right] > nums[largest] { largest = right; }

        if largest != root {
            nums.swap(root, largest);
            Self::heapify(nums, n, largest);  // fix the affected subtree
        }
    }
}