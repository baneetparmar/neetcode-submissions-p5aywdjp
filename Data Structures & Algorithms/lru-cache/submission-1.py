from collections import OrderedDict

class LRUCache:

    def __init__(self, capacity: int):
        self.capacity = capacity
        self.cache = OrderedDict()


    def get(self, key: int) -> int:
        if key in self.cache:
            val = self.cache[key]
            self.cache.move_to_end(key)
            return val
        else:
            return -1


    def put(self, key: int, value: int) -> None:
        if len(self.cache) == self.capacity and key not in self.cache:
            self.cache.popitem(last=False)
        self.cache[key] = value
        self.cache.move_to_end(key)
        
        
