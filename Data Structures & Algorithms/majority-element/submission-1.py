class Solution:
    def majorityElement(self, nums: List[int]) -> int:
        counter  = {}
        for n in nums:
            counter[n] = counter.get(n,0) + 1
            if counter[n] > len(nums)/2:
                return n
