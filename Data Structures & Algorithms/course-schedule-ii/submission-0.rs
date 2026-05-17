impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let n = num_courses as usize;
        let mut adj = vec![vec![]; n];
        let mut indegree = vec![0usize; n];

        for p in &prerequisites {
            adj[p[1] as usize].push(p[0] as usize);
            indegree[p[0] as usize] += 1;
        }

        let mut q: std::collections::VecDeque<usize> = indegree
            .iter()
            .enumerate()
            .filter(|&(_, &d)| d == 0)
            .map(|(i, _)| i)
            .collect();

        let mut order = vec![];
        while let Some(course) = q.pop_front() {
            order.push(course as i32);
            for &next in &adj[course] {
                indegree[next] -= 1;
                if indegree[next] == 0 {
                    q.push_back(next);
                }
            }
        }

        if order.len() == n { order } else { vec![] }
    }
}