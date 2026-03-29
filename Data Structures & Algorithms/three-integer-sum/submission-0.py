class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        output = []
        seen = {}
        nums.sort() 

        for i in range(len(nums) - 2):
            target = 0 - nums[i]
            j = i+1
            k = len(nums) - 1
            while j < k:

                if nums[j] + nums[k] > target:
                    k -= 1
                elif nums[j] + nums[k] < target:
                    j += 1
                else:
                    if  not (nums[i],nums[j],nums[k]) in seen:
                        output.append([nums[i],nums[j],nums[k]])
                        seen[(nums[i],nums[j],nums[k])] = True
                    j += 1
                    k -= 1
        return output

