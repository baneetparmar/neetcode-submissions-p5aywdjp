# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def removeNthFromEnd(self, head: Optional[ListNode], n: int) -> Optional[ListNode]:
        if not head or not head.next:
            head = None
            return head

        slow = head
        fast = head
        prev = slow
        # distance slow - fast pointers by n
        for _ in range(n):
            fast = fast.next
        while fast:
            prev = slow
            slow = slow.next
            fast = fast.next
        # if head is to be removed
        if prev == slow:
            head = head.next
        else:
            prev.next = slow.next
        
        return head
