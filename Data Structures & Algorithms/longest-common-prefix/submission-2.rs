impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() { return String::new(); }

        for (i, ch) in strs[0].chars().enumerate(){
            for word in &strs[1..]{
                if word.chars().nth(i) != Some(ch) {
                    return strs[0][..i].to_string();
                }
            }

        }
        strs[0].clone()
    }
}
