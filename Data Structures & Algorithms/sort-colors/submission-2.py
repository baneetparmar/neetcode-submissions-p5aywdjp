class Solution:
    def sortColors(self, nums: List[int]) -> None:
        count = defaultdict(int)
        for color in nums:
            count[color] += 1
        pos = 0
        for i in range(3):
            while count[i] > 0:
                nums[pos] = i
                count[i] -= 1
                pos += 1
        
            
