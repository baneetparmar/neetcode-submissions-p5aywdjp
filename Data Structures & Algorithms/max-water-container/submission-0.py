class Solution:
    def maxArea(self, heights: List[int]) -> int:
        l = 0
        r = len(heights) - 1
        stored = -1
        while l < r:
            stored = max(stored, (r - l) * min(heights[l],heights[r]))
            if heights[l] > heights[r]:
                r -= 1
            else:
                l += 1
        return stored
