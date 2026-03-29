impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut character_count = [0;26];
        for ch in s.chars(){
            character_count[(ch as u8 - b'a') as usize] += 1
        }
        for ch in t.chars(){
            character_count[(ch as u8 - b'a') as usize] -= 1
        }
        character_count.iter().all(|&count| count == 0)        
    }
}