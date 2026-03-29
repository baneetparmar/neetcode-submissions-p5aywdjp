class Solution:
    def maxSlidingWindow(self, nums: List[int], k: int) -> List[int]:
        l = 0
        r = k - 1
        m = None
        res = []
        while r < len(nums):
            if m is None or prev == m:
                m = max(nums[l:r + 1])
            else:
                m = max(m,nums[r])
            res.append(m)
            prev = nums[l]
            l += 1
            r += 1
        return res