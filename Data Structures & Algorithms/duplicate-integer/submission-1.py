class Solution:
    def hasDuplicate(self, nums: List[int]) -> bool:
        seen = []

        for n in nums:
            for m in seen:
                if n == m:
                    return True
            seen.append(n)

        return False