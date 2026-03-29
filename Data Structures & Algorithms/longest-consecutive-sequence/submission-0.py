class Solution:
    def longestConsecutive(self, nums: List[int]) -> int:
        seen = set()
        nums.sort()
        stack = []
        lc = 0
        for n in nums:
            if not n in seen:
                if not stack:
                    stack.append(n)
                else:
                    if stack[-1] + 1 == n:
                        stack.append(n)
                    else:
                        if len(stack) > lc:
                            lc = len(stack)                      
                        stack.clear()
                        stack.append(n)
                seen.add(n)
        return max(len(stack),lc)


        