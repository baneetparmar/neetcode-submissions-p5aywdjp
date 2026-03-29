# Doubly Linked List Node
class Node:
    def __init__(self,key=None,val=0,prev=None,next=None):
        self.val = val
        self.key = key
        self.prev = prev
        self.next = next

class LRUCache:

    def __init__(self, capacity: int):
        self.capacity = capacity
        self.cache = dict() # key : Node(value)
        self.left = Node()
        self.right = Node()
        self.left.next = self.right
        self.right.prev = self.left
    
    def move_to_end(self,key):
        node = self.cache[key]
        
        # Remove node
        prev = node.prev
        nxt = node.next
        prev.next = nxt
        nxt.prev = prev

        # Insert before dummy right
        oldprev = self.right.prev
        oldprev.next = node
        self.right.prev = node
        node.prev = oldprev
        node.next = self.right



    def remove_oldest(self):
        node = self.left.next
        self.left.next = node.next
        node.next.prev = self.left
        node.next = None
        node.prev = None
        del self.cache[node.key]

    def get(self, key: int) -> int:
        if key in self.cache:
            val = self.cache[key].val
            self.move_to_end(key)
            return val
        else:
            return -1
        

    def put(self, key: int, value: int) -> None:
        # remove oldest -> if capacity full and not updating key
        if len(self.cache) == self.capacity and key not in self.cache:
            self.remove_oldest()
        if key in self.cache:
            self.cache[key].val = value
            self.move_to_end(key)
        else:
            self.cache[key] = Node(key,value,self.right.prev,self.right)
            self.right.prev.next = self.cache[key]
            self.right.prev = self.cache[key]


