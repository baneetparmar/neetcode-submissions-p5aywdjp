class Solution:
    def trap(self, height: List[int]) -> int:
        prefix = []
        trapped_h20 = 0

        for n in height:
            if not prefix:
                prefix.append(n)
            else:
                prefix.append(max(prefix[-1],n))
        suffix = 0
        for i in range(len(height) - 1, -1 , -1):
            trapped_h20 += max(min(prefix[i],suffix) - height[i],0)
            if height[i] > suffix:
                suffix = height[i]
        return trapped_h20