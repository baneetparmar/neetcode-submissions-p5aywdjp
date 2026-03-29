impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut res = String::new();
        let word1:Vec<char> = word1.chars().collect();
        let word2:Vec<char> = word2.chars().collect();
        let n = word1.len().min(word2.len());
        for i in 0..n {
            res.push(word1[i]);
            res.push(word2[i]);
        }
        res.extend(&word1[n..]);
        res.extend(&word2[n..]);
        res
    }
}
