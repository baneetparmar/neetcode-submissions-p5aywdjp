class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        profit = 0
        minBuy = []
        m = None
        for n in prices:
            if m is None:
                m = n
            else:
                m = min(m,n)
            minBuy.append(m)
        for i in range(len(prices)):
            profit = max(profit,prices[i] - minBuy[i])
        return profit