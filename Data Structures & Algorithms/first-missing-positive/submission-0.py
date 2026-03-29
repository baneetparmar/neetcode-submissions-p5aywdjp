class Solution:
    def firstMissingPositive(self, nums: List[int]) -> int:
        numbers = set(nums)
        for i in range(1,1000001):
            if i not in numbers:
                return i
        