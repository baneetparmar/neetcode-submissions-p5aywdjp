class Solution:
    def rotate(self, nums: List[int], k: int) -> None:
        pos = defaultdict(list)
        for i in range(len(nums)):
            pos[nums[i]].append((i + k)%len(nums))
        for n in pos:
            for i in pos[n]:
                nums[i] = n