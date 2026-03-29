class Solution:
    def isValidSudoku(self, board: List[List[str]]) -> bool:
        box = [[ set() for _ in range(3)] for i in range(3)]

        for i in range(9):
            r = set()
            c = set()
            for j in range(9):
                re =  board[i][j]
                if re != ".":
                    if re in r  :
                        return False
                    else:
                        r.add(re)

                ce = board[j][i]
                if ce != ".":
                    if ce in c:
                        return False
                    else:
                        c.add(ce)

                if re !=".":
                    if re in box[i//3][j//3]:
                        return False
                    else:
                        box[i//3][j//3].add(re)

        return True