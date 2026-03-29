class Solution:
    def containsNearbyDuplicate(self, nums: List[int], k: int) -> bool:
        if k == 0:
            return False
        window = set()
        j = 0
        for n in nums:
            # check if duplicate
            if n in window:
                return True
            # check if window full
            if len(window) < k:
                # if not add elements
                window.add(n)
                continue
            # if full - remove first added ele and then add cur ele
            window.remove(nums[j])
            window.add(n)
            j += 1

        return False