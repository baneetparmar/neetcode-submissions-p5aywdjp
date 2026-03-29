class Solution:
    def largestRectangleArea(self, heights: List[int]) -> int:
        maxArea = 0
        for i,v in enumerate(heights):
            w = 0
            h = v
            for j in heights[i:]:
                w += 1
                h = min(h,j)
                maxArea = max(maxArea, h * w)
        return maxArea