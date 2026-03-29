class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        seen = {}
        for i,v in enumerate(nums):
            if target - v in seen:
                return sorted([i,seen.get(target - v)])
            else:
                seen[v] = i
        return []