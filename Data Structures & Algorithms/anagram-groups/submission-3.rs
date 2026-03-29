impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<[u8; 26], Vec<String>> = HashMap::new();

        for word in strs {
            let mut freq = [0u8; 26];
            for ch in word.chars() {
                freq[ch as usize - 'a' as usize] += 1;
            }
            map.entry(freq).or_default().push(word);
        }

        map.into_values().collect()
    }
}