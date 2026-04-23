impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let word: Vec<char> = word.chars().collect();
        let rows = board.len();
        let cols = board[0].len();
        let mut board = board;

        fn backtrack(board: &mut Vec<Vec<char>>, word: &[char], row: usize, col: usize, idx: usize) -> bool {
            if idx == word.len() { return true; }
            if row >= board.len() || col >= board[0].len() { return false; }
            if board[row][col] != word[idx] { return false; }

            let tmp = board[row][col];
            board[row][col] = '#';

            let found = [(-1,0),(1,0),(0,-1),(0,1)].iter().any(|(dr, dc)| {
                let r = row as i32 + dr;
                let c = col as i32 + dc;
                if r >= 0 && c >= 0 {
                    backtrack(board, word, r as usize, c as usize, idx + 1)
                } else {
                    false
                }
            });

            board[row][col] = tmp;
            found
        }

        for r in 0..rows {
            for c in 0..cols {
                if backtrack(&mut board, &word, r, c, 0) {
                    return true;
                }
            }
        }
        false
    }
}