class NumMatrix:

    def __init__(self, matrix: List[List[int]]):
        self.matrix = matrix
        self.psm = self.prefix_sum_matrix(self.matrix)

    def prefix_sum_matrix(self,matrix:List[List[int]]) -> List[List[int]]:
        ps_mat = []
        for x in range(len(matrix)):
            row = []
            cur_sum = 0
            for y in range(len(matrix[x])):
                cur_sum += matrix[x][y]
                row.append(cur_sum)
            ps_mat.append(row)
        return ps_mat


    def sumRegion(self, row1: int, col1: int, row2: int, col2: int) -> int:
        output = 0
        for r in range(row1, row2 + 1):
            al = self.psm[r][col2]
            ex = self.psm[r][col1 - 1] if col1 - 1 > -1 else 0
            output += al - ex
        return output

        

