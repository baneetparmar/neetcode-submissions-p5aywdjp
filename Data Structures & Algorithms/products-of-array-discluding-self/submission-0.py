class Solution:
    def productExceptSelf(self, nums: List[int]) -> List[int]:
        zeros = 0
        output = []
        pre_prod = []
        prod = 1
        prev = 1
        for n in nums:
            if n == 0:
                zeros += 1
            else:
                prod *= n
            pre_prod.append(prod)
        max_prod = pre_prod[len(nums) - 1]
        if zeros > 1:
            return [0] * len(nums)
        elif zeros == 1:
            return [max_prod if n == 0 else 0 for n in nums ]
        # no zeros
        else:
            for p in pre_prod:
                output.append(prev * (max_prod//p))
                prev = p
        return output