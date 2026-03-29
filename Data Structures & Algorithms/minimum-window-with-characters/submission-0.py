class Solution:
    def minWindow(self, s: str, t: str) -> str:
        if len(s) < len(t):
            return ""

        need = {}
        for c in t:
            need[c] = need.get(c, 0) + 1
        n = len(need.keys())

        h = 0
        have = defaultdict(int)

        l = 0
        res = (0,len(s) + 1)
        for r in range(len(s)):
            c = s[r]
            if c in need:
                have[c] += 1
                if have[c] == need[c]:
                    h += 1
            while h == n:
                if r - l + 1 < res[1] - res[0] + 1:
                    res = (l,r)
                if s[l] in need:
                    have[s[l]] -= 1
                    if have[s[l]] < need[s[l]]:
                        h -= 1
                l += 1


        return s[res[0]:res[1] + 1] if res[1] <= len(s) else ""