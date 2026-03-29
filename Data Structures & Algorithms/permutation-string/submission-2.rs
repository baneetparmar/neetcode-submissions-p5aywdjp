use std::collections::HashMap;
impl Solution {
    fn counter(s:&[u8]) -> HashMap<u8,usize> {
        let mut counter = HashMap::new();
        for &ch in s {
            *counter.entry(ch).or_insert(0) += 1;
        }
        counter
    }

    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s2.len() < s1.len() {
            return false
        }

        let mut left = 0;
        let mut right = 0;
        let mut h1:HashMap<u8,usize> = Self::counter(s1.as_bytes());

        for window in s2.as_bytes().windows(s1.len()) {
            if Self::counter(window) == h1 {
                return true
            }
        }
        false
    }
}
