"""
# Definition for a Node.
class Node:
    def __init__(self, x: int, next: 'Node' = None, random: 'Node' = None):
        self.val = int(x)
        self.next = next
        self.random = random
"""

class Solution:
    def copyRandomList(self, head: 'Optional[Node]') -> 'Optional[Node]':
        if not head:
            return None
        mp = {None:None}
        tail = head
        new_head = None
        while tail:
            mp[tail] = Node(tail.val,tail.next,tail.random)
            if new_head is None:
                new_head = mp[tail]
            tail = tail.next
        tail = head
        while tail:
            new_tail = mp[tail]
            new_tail.next = mp[new_tail.next]
            new_tail.random = mp[new_tail.random]
            tail = tail.next
        return new_head