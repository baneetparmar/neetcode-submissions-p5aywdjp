class Solution:
    def majorityElement(self, nums: List[int]) -> List[int]:
        counter = defaultdict(int)
        for n in nums:
            counter[n] += 1
        output = []
        for k in counter:
            if counter[k] > len(nums)/3:
                output.append(k)
        return output
        