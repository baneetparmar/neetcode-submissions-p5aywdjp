class StockSpanner:

    def __init__(self):
        self.stack = []
        self.count = -1


    def next(self, price: int) -> int:
        if not self.stack or self.stack[-1][1] > price:
            self.count += 1
            self.stack.append((self.count,price))
            return 1
        else:
            while self.stack and self.stack[-1][1] <= price:
                self.stack.pop()
            self.count += 1
            res = self.count - (self.stack[-1][0] if self.stack else -1)
            self.stack.append((self.count,price))
            return res