class Solution:
    def containsNearbyDuplicate(self, nums: List[int], k: int) -> bool:
        s = 0
        while s < len(nums):
            e = s + 1
            while e < len(nums) and e <= s + k:
                if nums[s] == nums[e]:
                    return True
                e += 1
            s += 1
        return False