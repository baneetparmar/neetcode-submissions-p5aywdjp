class Solution:
    def dailyTemperatures(self, t: List[int]) -> List[int]:
        res = [0] * len(t)
        stack = []
        for i in range(len(t)):
            if not stack or t[stack[-1]] > t[i]:
                stack.append(i)
            else:
                while stack and t[stack[-1]] < t[i]:
                    ele = stack.pop()
                    res[ele] = i - ele
                stack.append(i)
        return res
