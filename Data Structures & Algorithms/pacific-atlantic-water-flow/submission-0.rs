use std::collections::VecDeque;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (rows, cols) = (heights.len(), heights[0].len());
        let mut pacific = vec![vec![false; cols]; rows];
        let mut atlantic = vec![vec![false; cols]; rows];

        let mut pq = VecDeque::new();
        let mut aq = VecDeque::new();

        for r in 0..rows {
            pacific[r][0] = true;
            pq.push_back((r, 0));
            atlantic[r][cols-1] = true;
            aq.push_back((r, cols-1));
        }
        for c in 0..cols {
            pacific[0][c] = true;
            pq.push_back((0, c));
            atlantic[rows-1][c] = true;
            aq.push_back((rows-1, c));
        }

        Self::bfs(&heights, &mut pacific, &mut pq, rows, cols);
        Self::bfs(&heights, &mut atlantic, &mut aq, rows, cols);

        let mut res = vec![];
        for r in 0..rows {
            for c in 0..cols {
                if pacific[r][c] && atlantic[r][c] {
                    res.push(vec![r as i32, c as i32]);
                }
            }
        }
        res
    }

    fn bfs(
        heights: &Vec<Vec<i32>>,
        visited: &mut Vec<Vec<bool>>,
        q: &mut VecDeque<(usize, usize)>,
        rows: usize,
        cols: usize,
    ) {
        while let Some((r, c)) = q.pop_front() {
            for (nr, nc) in [
                (r.wrapping_sub(1), c), (r+1, c),
                (r, c.wrapping_sub(1)), (r, c+1)
            ] {
                if nr < rows && nc < cols && !visited[nr][nc] && heights[nr][nc] >= heights[r][c] {
                    visited[nr][nc] = true;
                    q.push_back((nr, nc));
                }
            }
        }
    }
}