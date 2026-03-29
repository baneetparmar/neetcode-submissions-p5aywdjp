class Solution:
    def longestConsecutive(self, nums: List[int]) -> int:
        hashset = set(nums)
        ls = 0
        for n in hashset:
            if not n - 1 in hashset:
                e = n
                cur_sq = 0
                while e in hashset:
                    cur_sq += 1
                    e += 1
                ls = max(cur_sq,ls)
        return ls
        


        