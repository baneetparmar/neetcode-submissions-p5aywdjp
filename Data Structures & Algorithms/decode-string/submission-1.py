class Solution:
    def stringify(self,num,txt):
        num = int(num[::-1])
        return num * txt[::-1]


    def decodeString(self, s: str) -> str:
        res = []
        stack = []

        for c in s:
            if c != "]":
                stack.append(c)
            else:
                t = ""
                while stack and stack[-1] != "[":
                    t += stack.pop()
                stack.pop()
                n = ""
                while stack and stack[-1].isdigit():
                    n += stack.pop()
                for nc in self.stringify(n,t):
                    stack.append(nc)
        return "".join(stack)