impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut in_deg = vec![0i32; n + 1];
        let mut out_deg = vec![0i32; n + 1];
        for t in &trust {
            out_deg[t[0] as usize] += 1;
            in_deg[t[1] as usize] += 1;
        }
        (1..=n as i32)
            .find(|&p| in_deg[p as usize] == n as i32 - 1 && out_deg[p as usize] == 0)
            .unwrap_or(-1)
    }
}