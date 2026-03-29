use std::collections::HashSet;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut left = 0;
        let mut longest_substring_length = 0;
        let s:Vec<char> = s.chars().collect();
        let mut set:HashSet<char> = HashSet::new();
        for right in 0..s.len(){
            while set.contains(&s[right]){
                set.remove(&s[left]);
                left += 1;
            }
            set.insert(s[right]);
            longest_substring_length = longest_substring_length.max(set.len());
        }
        longest_substring_length as i32
    }
}
