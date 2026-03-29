# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def reorderList(self, head: Optional[ListNode]) -> None:
        if not head or not head.next:
            return
        
        hm = {}
        curr = head
        count = 0
        
        while curr:
            hm[count] = curr
            curr = curr.next
            count += 1
        
        l, r = 0, count - 1
        prev = None
        
        while l <= r:
            if prev:
                prev.next = hm[l]
            prev = hm[l]
            
            if l == r:
                break
            
            prev.next = hm[r]
            prev = hm[r]
            
            l += 1
            r -= 1
        
        prev.next = None