type Board = Vec<Vec<char>>;

impl Solution {
    pub fn is_valid_sudoku(board: Board) -> bool {
        let mut rows  = vec![vec![false; 9]; 9];
        let mut cols  = vec![vec![false; 9]; 9];
        let mut boxes = vec![vec![false; 9]; 9];

        for r in 0..9 {
            for c in 0..9 {
                if board[r][c] == '.' { continue; }

                let d = (board[r][c] as u8 - b'1') as usize; // digit 0-8
                let b = (r / 3) * 3 + c / 3;                 // box index 0-8

                if rows[r][d]  { return false; }
                if cols[c][d]  { return false; }
                if boxes[b][d] { return false; }

                rows[r][d]  = true;
                cols[c][d]  = true;
                boxes[b][d] = true;
            }
        }
        true
    }
}
