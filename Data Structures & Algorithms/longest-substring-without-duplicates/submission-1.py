class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        window = set()
        i = 0
        longest = 0
        for c in s:
            if c in window:
                while c in window:
                    window.remove(s[i])
                    i += 1
            window.add(c)
            longest = max(longest,len(window))
        return longest
        