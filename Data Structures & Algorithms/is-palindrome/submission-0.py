class Solution:
    def isPalindrome(self, s: str) -> bool:
        strng = "".join([s if s.isalnum() else ""  for s in s.lower()])
        return strng == strng[::-1]
        