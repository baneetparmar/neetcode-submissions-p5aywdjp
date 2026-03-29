class Solution:
    def splitArray(self, nums: List[int], k: int) -> int:
        l = max(nums)
        r = sum(nums)

        mini = r

        while l <= r:
            m = l + (r - l)//2

            subarraycount = 1
            s = 0
            for n in nums:
                if s + n > m:
                    subarraycount += 1
                    s = 0
                s += n
            if subarraycount <= k:
                # current value good, can do better
                mini = min(mini,m)
                # move left
                r = m - 1
            else:
                l = m + 1
        return mini
