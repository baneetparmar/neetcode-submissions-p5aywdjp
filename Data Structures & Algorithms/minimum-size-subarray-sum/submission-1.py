class Solution:
    def minSubArrayLen(self, t: int, nums: List[int]) -> int:
        l = r = 0

        add = 0
        s = 2 * len(nums)
        while r < len(nums):
            add += nums[r]
            r += 1

            while add >= t:
                s = min(s, r - l)
                add -= nums[l]
                l += 1

        return s if s <= len(nums) else 0