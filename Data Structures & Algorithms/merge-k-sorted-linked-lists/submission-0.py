# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:    
    def mergeKLists(self, lists: List[Optional[ListNode]]) -> Optional[ListNode]:
        if not lists:
            return None
        vals = []
        for n in lists:
            while n:
                vals.append(n.val)
                n = n.next
        dummy = ListNode()
        tail = dummy
        vals.sort()
        for v in vals:
            tail.next = ListNode(v)
            tail = tail.next
        return dummy.next