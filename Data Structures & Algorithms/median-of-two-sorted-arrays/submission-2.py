class Solution:
    def findMedianSortedArrays(self, nums1: List[int], nums2: List[int]) -> float:
        # l is larger array and s is smaller array
        L,S = (nums1,nums2) if len(nums1) > len(nums2) else (nums2,nums1)

        l = 0
        r = len(S) - 1
        half = (len(L) + len(S))//2

        while True:
            m = l + (r - l)//2
            e = half - m - 2

            sLeft = S[m] if m >= 0 else float("-infinity")
            sRight = S[m + 1] if ( m + 1) < len(S) else float("infinity")
            lLeft = L[e] if e >= 0 else float("-infinity")
            lRight = L[e + 1] if ( e + 1) < len(L) else float("infinity")

            if sLeft <= lRight and lLeft <= sRight:
                if (len(L) + len(S)) % 2 == 0:
                    return (min(sRight,lRight) + max(lLeft,sLeft))/2
                else:
                    return min(lRight,sRight)
            elif sLeft > lRight:
                r = m - 1
            else:
                l = m + 1