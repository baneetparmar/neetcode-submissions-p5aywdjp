use std::collections::HashMap;

impl Solution {
    fn character_frequency(strng:String) -> HashMap<char,u32> {
        let mut output:HashMap<char,u32> = HashMap::new();
        for ch in strng.chars(){
            output.entry(ch).and_modify(|counter| *counter += 1).or_insert(1);
        };
        output
    }

    pub fn is_anagram(s: String, t: String) -> bool {
        // anagram have same characters and same string length
        // is both not equal they can't have same characters
        if s.len() != t.len(){
            return false
        }
        // check if the both have same character with same frequency
        let character_freq_for_s:HashMap<char,u32> = Self::character_frequency(s);
        let character_freq_for_t:HashMap<char,u32> = Self::character_frequency(t);
        character_freq_for_s == character_freq_for_t
    }
}
