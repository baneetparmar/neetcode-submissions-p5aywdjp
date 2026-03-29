class Solution:
    def hasDuplicate(self, nums: List[int]) -> bool:
        seen = {}
        for num in nums:
            if seen.get(num,False):
                return True
            seen[num] = True
        
        return False