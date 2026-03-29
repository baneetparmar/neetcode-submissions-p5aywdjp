class Solution:
    def rotate(self, nums: List[int], k: int) -> None:
        if len(nums) == 1:
            return
        rotated = [0]* len(nums)
        for i in range(len(nums)):
            rotated[(i+k)%len(nums)] = nums[i]
        nums[:] = rotated[:]