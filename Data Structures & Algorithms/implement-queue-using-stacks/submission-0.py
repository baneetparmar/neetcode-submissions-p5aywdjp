class MyQueue:

    def __init__(self):
        self.queue = list()
        self.tmp = list()

    def push(self, x: int) -> None:
        self.queue.append(x)

    def pop(self) -> int:
        while self.queue:
            self.tmp.append(self.queue.pop())

        first = self.tmp.pop()

        while self.tmp:
            self.queue.append(self.tmp.pop())

        return first

    def peek(self) -> int:
        return self.queue[0]

    def empty(self) -> bool:
        return len(self.queue) == 0