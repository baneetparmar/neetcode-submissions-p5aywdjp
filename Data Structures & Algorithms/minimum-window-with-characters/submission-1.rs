use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        if s.len() < t.len() { return "".to_owned() }

        let s:Vec<char> = s.chars().collect();

        let characters_in_t:HashMap<char,usize> = t.chars()
                            .fold(HashMap::new(),|mut map, ch| {
                                *map.entry(ch).or_insert(0) += 1;
                                map
                            } );
        let mut chars_req_to_match = characters_in_t.len(); // is number of unique characters in 't'.

        let mut left = 0;
        let mut chars_that_match = 0;
        let mut window = HashMap::<char,usize>::new();
        let mut minimum_valid_substring = (0,usize::MAX);

        for right in 0..s.len() {
            let ch = s[right];
            if characters_in_t.contains_key(&ch) {
                *window.entry(ch).or_insert(0) += 1;

                if window[&ch] == characters_in_t[&ch] {
                    chars_that_match += 1;
                }
            }

            while chars_req_to_match == chars_that_match {
            // shrink from left inorder to minimize
                if right - left < minimum_valid_substring.1 - minimum_valid_substring.0 {
                    minimum_valid_substring = (left,right);
                }

                if let Some(v) = window.get_mut(&s[left]) {
                    *v -= 1;
                    if *v < *characters_in_t.get(&s[left]).unwrap() {
                        chars_that_match -= 1;
                    }
                }

                // check if substring valid.
                left += 1;
            }
        }
        if minimum_valid_substring.1 == usize::MAX {
            "".to_owned()
        } else {
            String::from_iter(&s[minimum_valid_substring.0..=minimum_valid_substring.1])
        }
        
    }
}
