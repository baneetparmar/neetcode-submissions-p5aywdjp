impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set:HashSet<i32> = std::collections::HashSet::from_iter(nums.clone());
        let mut longest = 0;
        for &n in &set {
            if !set.contains(&(n - 1)){
                let mut seq_len = 0;
                while set.contains(&(n + seq_len)){
                    seq_len += 1
                }
                longest = longest.max(seq_len);
            } 
        }
        longest
    }
}
