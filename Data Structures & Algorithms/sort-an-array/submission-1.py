class Solution:
    
    def sortArray(self, nums: List[int]) -> List[int]:
        if len(nums) <= 1:
            return nums

        mid = len(nums)//2

        left,right = self.sortArray(nums[:mid]),self.sortArray(nums[mid:])

        i,j = 0,0

        output = []
        while i < len(left) and j < len(right):
            if left[i] < right[j]:
                output.append(left[i])
                i += 1
            else:
                output.append(right[j])
                j += 1
        output.extend(left[i:])
        output.extend(right[j:])
        return output      

        