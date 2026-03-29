impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let mut l = 0;
        let mut r = s.len() - 1;
        while l < r {
            //(s[l],s[r]) = (s[r],s[l]);
            s.swap(l,r);
            l += 1;
            r -= 1;
        }
    }
}
