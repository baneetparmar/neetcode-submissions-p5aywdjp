class Solution:
    def validate(self,s,del_allowed):
        l = 0
        r = len(s) - 1
        while l < r:
            if s[l] != s[r]:
                if del_allowed:
                    return self.validate(s[l+ 1:r + 1],False) or self.validate(s[l: r],False)
                return False
            l += 1
            r -= 1
        return True



    def validPalindrome(self, s: str) -> bool:
        return self.validate(s,True)