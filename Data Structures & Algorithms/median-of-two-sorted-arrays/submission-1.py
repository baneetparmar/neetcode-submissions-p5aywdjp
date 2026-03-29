class Solution:
    def findMedianSortedArrays(self, nums1: List[int], nums2: List[int]) -> float:
        merged = []
        i = 0
        j = 0
        for _ in range((len(nums1) + len(nums2))//2 + 1):
            if i >= len(nums1):
                merged.append(nums2[j])
                j += 1
                continue
            if j >= len(nums2):
                merged.append(nums1[i])
                i += 1
                continue
            
            if nums1[i] < nums2[j]:
                merged.append(nums1[i])
                i += 1
            else:
                merged.append(nums2[j])
                j += 1
        if (len(nums1) + len(nums2)) % 2 == 0:
            return (merged[-2] + merged[-1])/2
        else:
            return merged[-1]