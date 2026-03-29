type Matrix = Vec<Vec<i32>>;

struct NumMatrix {
    prefix:Matrix,
}

impl NumMatrix {
    fn new(matrix: Matrix) -> Self {
        let prefix = Self::build_prefix(&matrix);
        Self { prefix }
    }

    fn build_prefix(matrix:&Matrix) -> Matrix{
        let rows = matrix.len();
        let cols = matrix[0].len();
        let mut prefix = vec![vec![0; cols + 1]; rows + 1]; // pad top row and left col;

        for r in 1..=rows {
            for c in 1..=cols {
                prefix[r][c] = prefix[r - 1][c] + prefix[r][c - 1] - prefix[r - 1][c - 1] + matrix[r - 1][c - 1];
            }
        }
        prefix
    } 

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let (r1,c1,r2,c2) = (row1 as usize,col1 as usize, row2 as usize, col2 as usize);
        self.prefix[r2 + 1][c2 + 1] - self.prefix[r1][c2 + 1] - self.prefix[r2 + 1][c1] + self.prefix[r1][c1]
    }
}
