class Solution:
    def largestRectangleArea(self, heights: List[int]) -> int:
        maxArea = 0
        stack = []
        for idx,h in enumerate(heights):
            if not stack or stack[-1][1] < h:
                stack.append((idx,h))
            else:
                exi = h
                while stack and stack[-1][1] > h:
                    bar = stack.pop()
                    exi = bar[0]
                    area = bar[1] * (idx - bar[0])
                    maxArea = max(maxArea,area)
                stack.append((exi,h))
        while stack:
            bar = stack.pop()
            area = bar[1] * (idx - bar[0] + 1)
            maxArea = max(maxArea,area)
                
        return maxArea
