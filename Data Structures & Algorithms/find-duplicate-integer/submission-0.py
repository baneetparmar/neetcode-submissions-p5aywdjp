class Solution:
    def findDuplicate(self, nums: List[int]) -> int:
        slow = fast = 0
        # first point of intersection
        # of the cycle
        while True:
            slow = nums[slow]
            fast = nums[nums[fast]]
            if fast == slow:
                break

        second_slow = 0
        # search for second intersection
        while True:
            second_slow = nums[second_slow]
            slow = nums[slow]
            if second_slow == slow:
                return second_slow
        
