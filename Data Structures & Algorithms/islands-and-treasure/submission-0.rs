use std::collections::VecDeque;

impl Solution {
    pub fn islands_and_treasure(grid: &mut Vec<Vec<i32>>) {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut q = VecDeque::new();

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == 0 {
                    q.push_back((r, c));
                }
            }
        }

        while let Some((r, c)) = q.pop_front() {
            for (nr, nc) in [
                (r.wrapping_sub(1), c), (r+1, c),
                (r, c.wrapping_sub(1)), (r, c+1)
            ] {
                if nr < rows && nc < cols && grid[nr][nc] == i32::MAX {
                    grid[nr][nc] = grid[r][c] + 1;
                    q.push_back((nr, nc));
                }
            }
        }
    }
}