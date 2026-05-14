impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let (rows, cols) = (board.len(), board[0].len());

        for r in 0..rows {
            if board[r][0] == 'O'       { Self::dfs(board, r, 0, rows, cols); }
            if board[r][cols-1] == 'O'  { Self::dfs(board, r, cols-1, rows, cols); }
        }
        for c in 0..cols {
            if board[0][c] == 'O'       { Self::dfs(board, 0, c, rows, cols); }
            if board[rows-1][c] == 'O'  { Self::dfs(board, rows-1, c, rows, cols); }
        }

        for r in 0..rows {
            for c in 0..cols {
                match board[r][c] {
                    'O' => board[r][c] = 'X',
                    'S' => board[r][c] = 'O',
                    _   => {}
                }
            }
        }
    }

    fn dfs(board: &mut Vec<Vec<char>>, r: usize, c: usize, rows: usize, cols: usize) {
        if r >= rows || c >= cols || board[r][c] != 'O' { return; }
        board[r][c] = 'S';
        if r > 0 { Self::dfs(board, r-1, c, rows, cols); }
        Self::dfs(board, r+1, c, rows, cols);
        if c > 0 { Self::dfs(board, r, c-1, rows, cols); }
        Self::dfs(board, r, c+1, rows, cols);
    }
}