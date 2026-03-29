class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        if len(prices) <= 1:
            return 0

        buy = 0
        sell = 1
        profit = 0
        while sell < len(prices):
            if prices[sell] < prices[buy]:
                buy = sell
                sell += 1
            else:
                while sell < len(prices) and prices[sell - 1] < prices[sell]:
                    sell += 1
                profit += prices[sell - 1] - prices[buy]
                buy = sell
                sell += 1
        return profit
                        