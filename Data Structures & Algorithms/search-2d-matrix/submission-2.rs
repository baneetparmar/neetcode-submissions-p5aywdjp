impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {

        let rows = matrix.len();
        let cols = matrix[0].len();

        let mut t = 0;
        let mut b = rows;

        while t < b {
            let mid = t + (b - t)/2;
            if matrix[mid][0] <= target && target <= *matrix[mid].last().unwrap(){
                if let Ok(_) = matrix[mid].binary_search(&target) {
                    return true;
                } else {
                    return false;
                }
            } else if target < matrix[mid][0] {
                b = mid;
            } else {
                t = mid + 1;
            }
        }
        false
    }
}
