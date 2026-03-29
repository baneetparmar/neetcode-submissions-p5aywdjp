class NumMatrix:

    def __init__(self, matrix: List[List[int]]):
        self.matrix = self.prefix_sum_matrix(matrix)
        
    
    def prefix_sum_matrix(self,matrix):
        for r in range(len(matrix)):
            for c in range(len(matrix[r])):
                cur = matrix[r][c]
                up = matrix[r - 1][c] if r - 1 > -1 else 0
                left = matrix[r][ c - 1] if c - 1 > -1 else 0
                diag = matrix[r - 1][c - 1] if r-1> -1 and c-1 > -1 else 0
                matrix[r][c] = cur + up + left - diag
        return matrix
                

    def sumRegion(self, row1: int, col1: int, row2: int, col2: int) -> int:
        total = self.matrix[row2][col2]
        up = self.matrix[row1 - 1][col2] if row1 - 1 > -1 else 0
        left = self.matrix[row2][col1 - 1] if col1 - 1 > - 1 else 0
        overlap = self.matrix[row1 - 1][col1 - 1] if col1 -1 > -1 and row1 -1 > -1 else 0
        return total - up -left + overlap