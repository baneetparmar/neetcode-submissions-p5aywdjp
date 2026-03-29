impl Solution {
    pub fn decode_string(s: String) -> String {
        let mut count_stack:Vec<i32> = vec![];
        let mut string_stack:Vec<Vec<u8>> = vec![];
        let mut num = 0;
        let mut current:Vec<u8> = vec![];
        
        for &ch in s.as_bytes() {
            match ch {
                b'0'..=b'9' => num = num * 10 + (ch - b'0' ) as i32,
                b'[' => {
                    count_stack.push(num);
                    string_stack.push(current.clone());
                    current.clear();
                    num = 0;
                },
                b']' => {
                    let rep = count_stack.pop().unwrap();
                    let mut prev = string_stack.pop().unwrap();
                    prev.extend(current.repeat(rep as usize));
                    current = prev;
                },
                _ => current.push(ch),
            }
        }
        String::from_utf8(current).unwrap()
    }
}
