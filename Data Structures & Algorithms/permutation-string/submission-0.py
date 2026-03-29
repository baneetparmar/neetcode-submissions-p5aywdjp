class Solution:
    def checkInclusion(self, s1: str, s2: str) -> bool:
        from collections import Counter
        l = 0
        r = 0

        freq_s1 = Counter(s1)

        w = {}
        while r < len(s2):
            if s2[r] in w:
                w[s2[r]] += 1
            else:
                w[s2[r]] = 1

            if r + 1 >= len(s1):
                if w == freq_s1:
                    return True
                w[s2[l]] -= 1
                if w[s2[l]] == 0:
                    del w[s2[l]]
                l += 1
            r += 1
        
        return False
            

