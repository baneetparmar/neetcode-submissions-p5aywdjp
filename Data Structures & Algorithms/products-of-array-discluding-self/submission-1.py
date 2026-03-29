class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        prefix = []
        suffix = []
        output = []

        prod = 1
        prev = 1
        for n in nums:
            prod *= prev
            prefix.append(prod)
            prev = n

        prod = 1
        prev = 1
        for n in nums[::-1]:
            prod *= prev
            suffix.append(prod)
            prev = n
        for i in range(len(nums)):
            output.append(prefix[i] * suffix[len(nums) - 1 - i])
        return output


