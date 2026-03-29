# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def reverseBetween(self, head: Optional[ListNode], left: int, right: int) -> Optional[ListNode]:
        if not head or left == right:
            return head
        start = head
        end = head
        lstart = None
        # left: 1 indexed -> 0 indexed
        # move to position
        for _ in range(left - 1):
            lstart = start 
            start = start.next
        
        # right: 1 indexed -> 0 indexed
        # move to position
        for _ in range(right - 1):
            end = end.next

        # reverse
        # prev -> start -> ........ -> end -> after -> ......
        # prev -> end -> .......... -> start -> after -> .....
        prev = end.next # edge handled by contraints
        for _ in range(right - left + 1):
            nxt = start.next
            start.next = prev
            prev = start
            start = nxt
        
        # if reverse includes head
        # update head
        if left == 1:
            head = prev
        else:
            # head not touched
            lstart.next = prev
        return head

        
        