class Solution:
    def searchMatrix(self, matrix: List[List[int]], target: int) -> bool:
        t = 0
        b = len(matrix) - 1

        while t<=b:
            m = t + (b - t)//2

            x = 0
            y = len(matrix[m]) - 1

            while x <= y:
                im = x + (y - x)//2

                if matrix[m][im] == target:
                    return True
                elif matrix[m][im] < target:
                    x = im + 1
                else:
                    y = im - 1
            if matrix[m][0] < target < matrix[m][-1]:
                return False
            elif matrix[m][0] < target:
                t = m + 1
            else:
                b = m - 1
        return False