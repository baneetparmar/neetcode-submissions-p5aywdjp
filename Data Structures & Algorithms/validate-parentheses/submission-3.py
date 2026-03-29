class Solution:
    def isValid(self, s: str) -> bool:

        stack = []
        brackets = {
            '[':']',
            '{':'}',
            '(':')',
        }
        for c in s:
            if c in brackets:
                stack.append(c)
            else:
                if len(stack) > 0:
                    ele = stack.pop()
                    if brackets[ele] != c:
                        return False 
                   
                else:
                    return False
        return len(stack) == 0