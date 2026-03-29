class FreqStack:

    def __init__(self):
        self.stk = []
        self.counter = defaultdict(int)

    def push(self, val: int) -> None:
        self.counter[val] += 1
        while len(self.stk) <= self.counter[val]:
            self.stk.append([])
        self.stk[self.counter[val]].append(val)

    def pop(self) -> int:
        while not self.stk[-1]:
            self.stk.pop()
        val = self.stk[-1].pop()
        self.counter[val] -= 1
        return val
        
