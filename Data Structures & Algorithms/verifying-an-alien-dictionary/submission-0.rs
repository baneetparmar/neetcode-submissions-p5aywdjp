impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let mut rank = [0u8; 26];
        for (i, b) in order.bytes().enumerate() {
            rank[(b - b'a') as usize] = i as u8;
        }
        words.windows(2).all(|w| {
            let (a, b) = (w[0].as_bytes(), w[1].as_bytes());
            for i in 0..a.len().min(b.len()) {
                match rank[(a[i] - b'a') as usize].cmp(&rank[(b[i] - b'a') as usize]) {
                    std::cmp::Ordering::Less    => return true,
                    std::cmp::Ordering::Greater => return false,
                    std::cmp::Ordering::Equal   => {}
                }
            }
            a.len() <= b.len()
        })
    }
}