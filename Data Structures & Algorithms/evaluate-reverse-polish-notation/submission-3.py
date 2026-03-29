class Solution:
    def evalRPN(self, tokens: List[str]) -> int:
        token = tokens.pop()
        if token not in "+-*/":
            return int(token)
        
        right = self.evalRPN(tokens)
        left = self.evalRPN(tokens)

        match token:
            case "+":
                return left + right
            case "-":
                return left - right
            case "*":
                return left * right
            case "/":
                return int(left / right)
