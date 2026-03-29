class MyStack:
    from collections import deque
    def __init__(self):
        self.stack = deque()
        self.tmp = deque()

    def push(self, x: int) -> None:
        self.stack.append(x)

    def pop(self) -> int:
        while self.stack:
            ele = self.stack.popleft()
            if len(self.stack) == 0:
                while self.tmp:
                    self.stack.append(self.tmp.popleft())
                return ele
            else:
                self.tmp.append(ele)

    def top(self) -> int:
        return self.stack[-1]

    def empty(self) -> bool:
        return len(self.stack) == 0
    