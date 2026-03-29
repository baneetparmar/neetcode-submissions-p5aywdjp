class Solution:
    def majorityElement(self, nums: List[int]) -> List[int]:
        if len(nums) < 3:
            return nums
        c1,c2 = None,None
        cnt1,cnt2 = 0,0

        for n in nums:
            if n == c1:
                cnt1 += 1
            elif n == c2:
                cnt2 += 1
            else:
                if cnt1 == 0:
                    c1 = n
                    cnt1 = 1
                elif cnt2 == 0:
                    c2 = n 
                    cnt2 = 1
                else:
                    cnt1 -= 1
                    cnt2 -= 1


        output = []
        c0_count = 0
        c1_count = 0
        for n in nums:
            if n == c1:
               c0_count += 1
            if n == c2:
               c1_count += 1
        if c0_count > len(nums)/3: output.append(c1)
        if c1_count > len(nums)/3: output.append(c2)
        return output


