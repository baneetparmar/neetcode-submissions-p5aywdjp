class MyHashMap:

    def __init__(self):
        self.map = []

    def put(self, key: int, value: int) -> None:
        for i in self.map:
            if i[0] == key:
                i[1] = value
                break
        else:
            self.map.append([key,value])
        
        

    def get(self, key: int) -> int:
        for i in self.map:
            if i[0] == key:
                return i[1]
        return -1
        

    def remove(self, key: int) -> None:
        for i in self.map:
            if i[0] == key:
                self.map.remove(i)
        