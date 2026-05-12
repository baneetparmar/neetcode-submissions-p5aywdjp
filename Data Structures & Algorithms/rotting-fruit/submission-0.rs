use std::collections::VecDeque;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut q = VecDeque::new();
        let mut fresh = 0;

        for r in 0..rows {
            for c in 0..cols {
                match grid[r][c] {
                    2 => q.push_back((r, c, 0)),
                    1 => fresh += 1,
                    _ => {}
                }
            }
        }

        let mut time = 0;
        while let Some((r, c, t)) = q.pop_front() {
            for (nr, nc) in [
                (r.wrapping_sub(1), c), (r+1, c),
                (r, c.wrapping_sub(1)), (r, c+1)
            ] {
                if nr < rows && nc < cols && grid[nr][nc] == 1 {
                    grid[nr][nc] = 2;
                    fresh -= 1;
                    time = time.max(t + 1);
                    q.push_back((nr, nc, t + 1));
                }
            }
        }

        if fresh > 0 { -1 } else { time }
    }
}