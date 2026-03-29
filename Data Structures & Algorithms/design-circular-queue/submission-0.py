class Node():
    def __init__(self,val=0,next=None):
        self.val = val
        self.next = next

class MyCircularQueue:

    def __init__(self, k: int):
        self.capacity = k
        self.size = 0
        self.front = None
        self.rear = None

    def enQueue(self, value: int) -> bool:
        # queue already full -> return False
        # else: Add Node
        if self.size == self.capacity:
            return False

        # if first Node
        # create node -> point head 
        # point rear to front
        # increment size
        # create node and point self.rear to it
        if self.rear is None:
            self.rear = Node(value)
            self.front = self.rear

        # add node to end
        else:
            self.rear.next = Node(value)
            # update rear
            self.rear = self.rear.next

        self.rear.next = self.front            
        self.size += 1
        # all good
        return True
        

    def deQueue(self) -> bool:
        # no element to remove
        if self.size == 0:
            return False

        # queue becomes empty if 1 element in queue
        elif self.size == 1:
            self.front = None
            self.rear = None

        else:
            # remove element from front
            nxt = self.front.next
            self.front.next = None
            self.front = nxt
            self.rear.next = self.front

        self.size -= 1
        return True
        

    def Front(self) -> int:
        if self.size == 0:
            return -1
        return self.front.val
        
    def Rear(self) -> int:
        if self.size == 0:
            return -1
        return self.rear.val
        

    def isEmpty(self) -> bool:
        return self.size == 0
        

    def isFull(self) -> bool:
        return self.size == self.capacity
        
