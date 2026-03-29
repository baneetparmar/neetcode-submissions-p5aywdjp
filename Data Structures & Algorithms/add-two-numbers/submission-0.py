# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        dummy = ListNode()
        tail = dummy
        carry = 0
        while l1 and l2:
            sum = l1.val + l2.val + carry
            digit = sum % 10
            carry = sum // 10
            tail.next = ListNode(digit)
            tail = tail.next
            l1 = l1.next
            l2 = l2.next


        if l1 and not l2:
            while l1 and carry:
                sum = l1.val + carry
                digit = sum % 10
                carry = sum // 10
                tail.next = ListNode(digit)
                tail = tail.next
                l1 = l1.next
            while l1:
                tail.next = l1
                tail = tail.next
                l1 = l1.next

        elif l2 and not l1:
            while l2 and carry:
                sum = l2.val + carry
                digit = sum % 10
                carry = sum // 10
                tail.next = ListNode(digit)
                tail = tail.next
                l2 = l2.next
            while l2:
                tail.next = l2
                tail = tail.next
                l2 = l2.next
        if carry:
            tail.next = ListNode(carry)
        return dummy.next
                

