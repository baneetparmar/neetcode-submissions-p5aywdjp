class Solution:
    def validate(self,s):
        l = 0
        r = len(s) - 1
        while l < r:
            if s[l] != s[r]:
                return False
            l += 1
            r -= 1
        return True



    def validPalindrome(self, s: str) -> bool:
        del_allowed = True
        l = 0
        r = len(s) - 1
        while l < r:
            if s[l] != s[r] and del_allowed is True:
                del_allowed = False
                return self.validate(s[l+ 1:r + 1]) or self.validate(s[l: r])
            l += 1
            r -= 1
        return True
                