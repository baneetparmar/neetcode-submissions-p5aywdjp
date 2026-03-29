impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack:Vec<u8> = vec![];
        let brackets:HashMap<u8,u8> = [(b'{',b'}'),(b'(',b')'),(b'[',b']')]
                                        .into_iter().collect();

        for &ch in s.as_bytes() {
            if let Some(&expected) = brackets.get(&ch){
                stack.push(expected);
            } else if stack.pop() != Some(ch) {
                return false
            }
        }
        stack.is_empty()
    }
}
