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
        if s2.len() < s1.len() { return false }

        let s2 = s2.as_bytes();
        let mut h1 = Self::counter(s1.as_bytes());

        let mut left = 0;
        let mut right = 0;
        let mut h2:HashMap<u8,usize> = HashMap::new();
        while right < s2.len() {
            *h2.entry(s2[right]).or_insert(0) += 1;
            if right >= s1.len() - 1{
                if h1 == h2 {
                    return true
                } else {
                    *h2.get_mut(&s2[left]).unwrap() -= 1;
                    if h2[&s2[left]] == 0 { h2.remove(&s2[left]); }
                    left += 1;
                }
            }
            right += 1;
        }
        false
    }
}
