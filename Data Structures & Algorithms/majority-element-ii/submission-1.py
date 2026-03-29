class Solution:
    def majorityElement(self, nums: List[int]) -> List[int]:
        nums.sort()
        output = []
        count = 0
        n = 0
        while n < len(nums):
            if n == 0 or nums[n] != nums[n - 1]:
                k = 0
                while n + k < len(nums) and nums[n + k] == nums[n]:
                    count += 1
                    k += 1
                if count > len(nums)/3:
                    output.append(nums[n])
                count = 0
                n = n + k
        return output
        