impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let delimiter = '#';
        let mut res = String::new();
        for st in strs{
            let encoding = format!("{}{}{}",st.len(),delimiter,st);
            res.push_str(&encoding);
        }
        res
    }

    pub fn decode(s: String) -> Vec<String> {
        let delimiter = b'#';
        let mut res = vec![];
        let mut head = 0;
        let bytes = s.as_bytes();

        while head < s.len(){
            let mut num = String::new();
            while bytes[head] != delimiter {
                num.push(bytes[head] as char);
                head += 1;
            }
            // head at delimiter;
            let length = num.parse::<usize>().unwrap();
            head += 1;
            let tail = head + length; // tail now points to start of new encoding;
            res.push(s[head..tail].to_string());
            head = tail; // now head at start of new encoding;
        }
        res
    }
}
