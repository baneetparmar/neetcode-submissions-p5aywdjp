impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s:Vec<u8> = s.bytes().filter(|ch| ch.is_ascii_alphanumeric()).map(|ch| ch.to_ascii_lowercase()).collect();
        if s.is_empty() { return true }
        let mut l = 0;
        let mut r = s.len() - 1;
        while l < r {
            if s[l] != s[r] {
                return false
            }
            l += 1;
            r -= 1;
        }
        true
    }
}
