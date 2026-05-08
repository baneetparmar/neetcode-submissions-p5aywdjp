use std::collections::VecDeque;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut grid = grid;
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut count = 0;
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == '1' {
                    count += 1;
                    grid[r][c] = '0';
                    let mut q = VecDeque::from([(r, c)]);
                    while let Some((r, c)) = q.pop_front() {
                        for (nr, nc) in [
                            (r.wrapping_sub(1), c), (r+1, c),
                            (r, c.wrapping_sub(1)), (r, c+1)
                        ] {
                            if nr < rows && nc < cols && grid[nr][nc] == '1' {
                                grid[nr][nc] = '0';
                                q.push_back((nr, nc));
                            }
                        }
                    }
                }
            }
        }
        count
    }
}