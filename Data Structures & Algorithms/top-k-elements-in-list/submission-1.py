class Solution:
    def topKFrequent(self, nums: List[int], k: int) -> List[int]:
        count = defaultdict(int)
        bucket = [[] for i in range(len(nums) + 1)]

        for n in nums:
            count[n] += 1
        
        for n,c in count.items():
            bucket[c].append(n)
        
        res = []
        for i in range(len(nums), -1 , -1): # n...=0
            for v in bucket[i]:
                res.append(v)
                if len(res) == k:
                    return res