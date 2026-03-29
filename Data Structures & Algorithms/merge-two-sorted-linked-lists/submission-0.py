# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:
    def mergeTwoLists(self, list1: Optional[ListNode], list2: Optional[ListNode]) -> Optional[ListNode]:
        if not list1:
            return list2
        if not list2:
            return list1
            
        head = merged = ListNode(None)
        while list1 is not None and list2 is not None:
            if list1.val <= list2.val:
                if merged.val is None:
                    merged.val = list1.val
                else:
                    merged.next = ListNode(list1.val)
                    merged = merged.next
                list1 = list1.next
            else:
                if merged.val is None:
                    merged.val = list2.val
                else:
                    merged.next = ListNode(list2.val)
                    merged = merged.next
                list2 = list2.next
        while list1 is not None:
            merged.next = ListNode(list1.val)
            list1 = list1.next
            merged = merged.next
        while list2 is not None:
            merged.next = ListNode(list2.val)
            list2 = list2.next
            merged = merged.next
        return head
