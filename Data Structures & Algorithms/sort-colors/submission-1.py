class Solution:
    def sortColors(self, nums: List[int]) -> None:
        for i in range(len(nums) - 1):
            smallest_at = i
            for j in range(i,len(nums)):
                if nums[j] < nums[smallest_at]:
                    smallest_at = j
            nums[i],nums[smallest_at] = nums[smallest_at],nums[i]
