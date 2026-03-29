class NumMatrix:

    def __init__(self, matrix: List[List[int]]):
        self.matrix = matrix
        

    def sumRegion(self, row1: int, col1: int, row2: int, col2: int) -> int:
        output = 0
        for r in range(row1, row2 + 1):
            for c in range(col1, col2 + 1):
                output += self.matrix[r][c]
        return output

        

