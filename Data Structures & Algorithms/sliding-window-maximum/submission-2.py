class Solution:
    import collections

    def maxSlidingWindow(self, nums: List[int], k: int) -> List[int]:

        l = r = 0
        deq = collections.deque()
        res = []

        while r < len(nums):
            while deq and nums[deq[-1]] < nums[r]:
                deq.pop()
            
            deq.append(r)

            if l > deq[0]:
                deq.popleft()
            
            if (r + 1) >= k:
                res.append(nums[deq[0]])
                l += 1
            r += 1
        return res