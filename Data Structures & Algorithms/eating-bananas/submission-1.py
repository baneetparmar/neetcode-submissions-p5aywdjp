class Solution:
    def minEatingSpeed(self, piles: List[int], h: int) -> int:
        lr = 1
        hr = max(piles)
        minreq = hr

        while lr <= hr:
            tt = 0
            m = lr + (hr - lr)//2

            for n in piles:
                tt += math.ceil(n/m)
            if tt <= h:
                minreq = min(minreq,m)

            if tt <= h: # eating too fast can slow down
                hr = m - 1
            else: # eating too slow hurry up monkey
                lr = m + 1
        return minreq