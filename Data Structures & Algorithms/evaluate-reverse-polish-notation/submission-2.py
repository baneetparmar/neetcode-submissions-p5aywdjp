class Solution:

    def solve(self, l,r,o):
        match o:
            case "+":
                return l + r
            case "-":
                return l - r
            case "*":
                return l * r
            case "/":
                return int(l / r)


    def evalRPN(self, tokens: List[str]) -> int:
        stack = []
        
        t = {'+','-','*','/'}
        for c in tokens:
            if c in t:
                r = stack.pop()
                l = stack.pop()
                stack.append(self.solve(l,r,c))
            else:
                stack.append(int(c))
        return stack.pop()