# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def reorderList(self, head: Optional[ListNode]) -> None:
        if not head or not head.next:
            return
        
        slow = head
        fast = head.next
        # find mid and split into two halves
        while fast and fast.next:
            slow = slow.next
            fast = fast.next.next
        
        second = slow.next
        # cut-off Node
        prev = slow.next = None
        # reverse the second half of list
        while second:
            tmp = second.next
            second.next = prev
            prev = second
            second = tmp
        
        first = head
        second = prev
        #merge two halfves 
        while second:
            tmp1, tmp2 = first.next,second.next
            first.next = second
            second.next = tmp1
            first = tmp1
            second = tmp2
        