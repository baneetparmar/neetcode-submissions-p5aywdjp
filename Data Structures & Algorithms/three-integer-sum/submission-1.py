class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        nums.sort()

        output = []

        for i in range(len(nums) - 2):
            if i > 0 and nums[i] == nums[i - 1]:
                continue

            target = - nums[i]
            j = i+1
            k = len(nums) - 1
            while j < k:
                if nums[j] + nums[k] > target:
                    k -= 1
                elif nums[j] + nums[k] < target:
                    j += 1
                else:
                    output.append([nums[i],nums[j],nums[k]])
                    while j < k and nums[k] == nums[k - 1]:
                        k -= 1
                    j += 1
                    k -= 1
        return output

