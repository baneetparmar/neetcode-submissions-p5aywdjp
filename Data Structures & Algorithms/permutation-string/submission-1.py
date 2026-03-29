class Solution:
    def checkInclusion(self, s1: str, s2: str) -> bool:
        fs1 = [0] * 26
        w = [0] * 26

        for c in s1:
            fs1[ord(c) - ord('a')] += 1
        
        l = 0
        for r in range(len(s2)):
            w[ord(s2[r]) - ord('a')] += 1
            if r + 1 >= len(s1):
                if w == fs1:
                    return True
                w[ord(s2[l]) - ord('a')] -= 1
                l += 1
        
        return False


