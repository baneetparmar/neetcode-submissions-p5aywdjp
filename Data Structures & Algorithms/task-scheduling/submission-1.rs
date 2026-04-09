impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut freq = [0i32; 26];
        for t in tasks.iter() {
            freq[*t as usize - 'A' as usize] += 1;
        }

        let max_f = *freq.iter().max().unwrap();
        let count_max_f = freq.iter().filter(|&&f| f == max_f).count() as i32;
        let ans = (max_f - 1) * (n + 1) + count_max_f;

        ans.max(tasks.len() as i32)
    }
}
