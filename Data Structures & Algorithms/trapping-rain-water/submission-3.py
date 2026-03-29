class Solution:
    def trap(self, height: List[int]) -> int:
        trapped_h20 = 0

        l = 0
        r = len(height) - 1
        maxL = 0
        maxR = 0

        while l < r:
            if height[l] < height[r]:
                trapped_h20 += max(maxL - height[l],0)
                if height[l] > maxL:
                    maxL = height[l]
                l += 1
            else:
                trapped_h20 += max(maxR - height[r],0)
                if height[r] > maxR:
                    maxR = height[r]

                r -= 1

        return trapped_h20