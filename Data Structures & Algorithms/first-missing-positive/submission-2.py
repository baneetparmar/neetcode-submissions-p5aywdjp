class Solution:
    def firstMissingPositive(self, nums: List[int]) -> int:
        
        i = 0
        max_val = len(nums)

        while i < len(nums):

            # check bounds
            if 0 < nums[i] <= max_val:

                # check duplicates
                ele = nums[i]
                if nums[ele - 1] == ele:
                    i += 1
                # checks cleared
                else:
                    nums[ele - 1],nums[i] = nums[i],nums[ele - 1]
            else:
                i += 1
        for i,v in enumerate(nums):
            if i + 1 != v:
                return i + 1
                
        # no element missing
        return nums[-1] + 1 
