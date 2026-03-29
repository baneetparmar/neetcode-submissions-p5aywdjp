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
        nodes = []
        curr = head
        while curr:
            nodes.append(curr)
            curr = curr.next

        if n != len(nodes):
            prev = len(nodes) - n - 1
            curr = len(nodes) - n
            nodes[prev].next = nodes[curr].next
        else:
            head = head.next
        
        return head
