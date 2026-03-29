class Solution:
    def isValidSudoku(self, board: List[List[str]]) -> bool:
        box = [[[0] for i in range(3)] for i in range(3)]

        for i in range(9):
            r = []
            c = []
            for j in range(9):
                re =  board[i][j]
                if re != ".":
                    if re in r or re in box[i//3][j//3] :
                        return False
                    else:
                        r.append(re)
                        box[i//3][j//3].append(re)

                ce = board[j][i]
                if ce != ".":
                    if ce in c:
                        return False
                    else:
                        c.append(ce)

        return True