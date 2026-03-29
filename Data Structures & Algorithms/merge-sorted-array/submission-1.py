class Solution:
    def merge(self, nums1: List[int], m: int, nums2: List[int], n: int) -> None:
        output = []
        l, r = 0, 0
        while l + r < len(nums1):
            if r == n:
                output.append(nums1[l])
                l += 1
            elif l == m:
                output.append(nums2[r])
                r += 1
            else:
                if nums1[l] <= nums2[r]:
                    output.append(nums1[l])
                    l += 1
                else:
                    output.append(nums2[r])
                    r += 1
        for i in range(len(output)):
            nums1[i] = output[i]

