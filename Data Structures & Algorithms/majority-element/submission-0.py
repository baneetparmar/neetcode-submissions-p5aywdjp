class Solution:
    def majorityElement(self, nums: List[int]) -> int:
        counter  = {}
        for n in nums:
            counter[n] = counter.get(n,0) + 1
        for k,v in counter.items():
            if v > len(nums)/2:
                return k