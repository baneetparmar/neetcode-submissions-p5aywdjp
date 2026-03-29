class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        profit = 0
        minBuy = []
        m = prices[0]
        for n in prices:
            profit = max(profit,n - m)
            m = min(m,n)
        return profit