class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        i = 0
        j = len(nums) - 1
        sorted_nums = sorted(nums)
        pos = []

        while True:
            if sorted_nums[i] + sorted_nums[j] == target:
                break
            elif sorted_nums[i] + sorted_nums[j] > target:
                j -= 1
            elif sorted_nums[i] + sorted_nums[j] < target:
                i += 1
        
        for p,v in enumerate(nums):
            if v == sorted_nums[i]:
                pos.append(p)
                break
        for p,v in enumerate(nums):
            if v == sorted_nums[j] and p != pos[0]:
                pos.append(p)
                break
        return sorted(pos)