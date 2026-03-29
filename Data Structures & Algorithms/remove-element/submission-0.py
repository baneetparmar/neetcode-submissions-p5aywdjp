class Solution:
    def removeElement(self, nums: List[int], val: int) -> int:
        s = 0
        e = len(nums)
        while s < e:
            if nums[s] == val:
                e -= 1
                nums[s] = nums[e]
            else:
                s += 1
        return e