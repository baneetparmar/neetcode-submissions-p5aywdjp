class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        counter = defaultdict(int)
        for n in nums:
            counter[n] += 1
        
        output = sorted(counter.keys(),key=counter.get,reverse=True)
        return output[:k]