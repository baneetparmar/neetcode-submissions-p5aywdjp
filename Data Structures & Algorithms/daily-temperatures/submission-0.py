class Solution:
    def dailyTemperatures(self, t: List[int]) -> List[int]:
        res = []
        for i in range(len(t)):
            d = 0
            for j in range(i,len(t)):
                if t[j] > t[i]:
                    d = j - i
                    break
            else:
                d = 0
            res.append(d)
        return res
