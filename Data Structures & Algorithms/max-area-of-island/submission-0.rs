impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut max = 0;
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == 1 {
                    max = max.max(Self::dfs(&mut grid, r, c, rows, cols));
                }
            }
        }
        max
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, r: usize, c: usize, rows: usize, cols: usize) -> i32 {
        if r >= rows || c >= cols || grid[r][c] != 1 { return 0; }
        grid[r][c] = 0;
        1 + Self::dfs(grid, r+1, c, rows, cols)
          + Self::dfs(grid, r, c+1, rows, cols)
          + if r > 0 { Self::dfs(grid, r-1, c, rows, cols) } else { 0 }
          + if c > 0 { Self::dfs(grid, r, c-1, rows, cols) } else { 0 }
    }
}