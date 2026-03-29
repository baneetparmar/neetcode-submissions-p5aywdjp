class Solution:
    def mySqrt(self, x: int) -> int:
        l = 1
        r = x//2 + 1

        while l <= r:
            m = l + (r - l)//2
            m_sq = m * m
            if m_sq == x:
                return m
            elif m_sq < x:
                l = m + 1
            else:
                r = m - 1
        return r