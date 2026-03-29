class Solution:
    def search(self, nums: List[int], target: int) -> int:
        l = 0
        r = len(nums) - 1

        while l <= r:
            m = l + (r - l)//2
            
            # best case target is mid
            if target == nums[m]:
                return True
            # if can't decide sorted -> shrink range
            elif nums[l] == nums[m] == nums[r]:
                l += 1
                r -= 1
            # check what is sorted
            # left sorted
            elif nums[l] <= nums[m]:
                # means target in left -> move left
                if nums[l] <= target < nums[m]:
                    r = m - 1
                # target not in left -> move right
                else:
                    l = m + 1
            # right sorted
            # right > mid
            else:
                # target > mid -> target in right of mid -> move right
                if nums[r] >= target > nums[m]:
                    l = m + 1
                # target not in right -> move left
                else:
                    r = m - 1
        return False