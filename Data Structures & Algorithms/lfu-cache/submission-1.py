from collections import defaultdict


class Node:
    def __init__(self, val=0):
        self.val = val
        self.prev = None
        self.next = None


class LinkedList:
    def __init__(self):
        self.left = Node()
        self.right = Node()
        self.left.next = self.right
        self.right.prev = self.left
        self.bucket = {}

    def length(self):
        return len(self.bucket)

    def push(self, val):
        node = Node(val)
        self.bucket[val] = node

        oldPrev = self.right.prev
        oldPrev.next = node
        node.prev = oldPrev
        node.next = self.right
        self.right.prev = node

    def pop(self, val):
        if val in self.bucket:
            node = self.bucket[val]
            prev = node.prev
            nxt = node.next

            prev.next = nxt
            nxt.prev = prev

            node.prev = None
            node.next = None

            del self.bucket[val]

    def popLeft(self):
        node = self.left.next
        val = node.val

        self.pop(val)
        return val

    def update(self, val):
        self.pop(val)
        self.push(val)


class LFUCache:

    def __init__(self, capacity: int):
        self.capacity = capacity
        self.minFreq = 0
        self.cache = {}                 # key -> value
        self.freq = defaultdict(int)    # key -> frequency
        self.cacheList = defaultdict(LinkedList)  # freq -> LinkedList

    def update_freq(self, key):
        f = self.freq[key]

        # Remove from old frequency bucket
        self.cacheList[f].pop(key)

        # Increase frequency
        self.freq[key] += 1
        self.cacheList[f + 1].push(key)

        # Update minFreq if needed
        if f == self.minFreq and self.cacheList[f].length() == 0:
            self.minFreq += 1

    def get(self, key: int) -> int:
        if key not in self.cache:
            return -1

        self.update_freq(key)
        return self.cache[key]

    def put(self, key: int, value: int) -> None:
        if self.capacity == 0:
            return

        # Update existing key
        if key in self.cache:
            self.cache[key] = value
            self.update_freq(key)
            return

        # Evict if full
        if len(self.cache) == self.capacity:
            evict = self.cacheList[self.minFreq].popLeft()
            del self.cache[evict]
            del self.freq[evict]

        # Insert new key
        self.cache[key] = value
        self.freq[key] = 1
        self.cacheList[1].push(key)
        self.minFreq = 1