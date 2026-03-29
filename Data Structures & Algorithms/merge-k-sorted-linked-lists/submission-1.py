# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next

class Solution:    
    def mergeKLists(self, lists: List[Optional[ListNode]]) -> Optional[ListNode]:
        if not lists:
            return None
        
        dummy = ListNode()
        tail = dummy

        while True:
            mini = None

            for i in range(len(lists)):
                if not lists[i]:
                    continue
                if mini is None or lists[i].val < lists[mini].val:
                    mini = i
            if mini is None:
                break
            
            tail.next = lists[mini]
            lists[mini] = lists[mini].next
            tail = tail.next
        return dummy.next