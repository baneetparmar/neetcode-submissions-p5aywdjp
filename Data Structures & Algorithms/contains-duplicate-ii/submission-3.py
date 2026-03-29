class Solution:
    def containsNearbyDuplicate(self, nums: List[int], k: int) -> bool:
        seen = {}
        for i in range(len(nums)):
            n = nums[i]
            if n in seen:
                if abs(seen[n] - i ) <= k:
                    return True
                seen[n] = i
            else:
                seen[n] = i
        return False