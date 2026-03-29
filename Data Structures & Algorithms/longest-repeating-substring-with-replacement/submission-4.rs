use std::collections::HashMap;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s:Vec<char> = s.chars().collect();
        let k = k as usize;

        let mut left = 0;
        let mut longest_substring = 0;
        let mut maximum_frequency = 0;
        let mut window:HashMap<char,usize> = HashMap::new();

        for right in 0..s.len() {
            let element = window.entry(s[right]).or_insert(0);
            *element += 1;
            maximum_frequency = maximum_frequency.max(*element);
            while ( right - left + 1) - maximum_frequency > k {
                let element = window.get_mut(&s[left]).unwrap();
                *element -= 1;
                left += 1;
            }
            longest_substring = longest_substring.max(right - left + 1);
        }
        longest_substring as i32
    }
}
