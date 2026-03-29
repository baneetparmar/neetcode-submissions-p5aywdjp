class Solution:
    def findMedianSortedArrays(self, nums1: List[int], nums2: List[int]) -> float:
        merged = sorted(nums1 + nums2)
        # even values
        if len(merged) % 2 == 0:
            return (merged[len(merged)//2] + merged[len(merged)//2 - 1])/2
        else:
            print()
            return merged[(len(merged) - 1)//2]