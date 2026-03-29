class TimeMap:

    def __init__(self):
        self.timemap = dict()

    def set(self, key: str, value: str, timestamp: int) -> None:
        if key in self.timemap:
            self.timemap[key].append([value,timestamp])
        else:
            self.timemap[key] = [[value,timestamp]]

    def get(self, key: str, timestamp: int) -> str:
        if key not in self.timemap:
            return ""
        l = 0
        r = len(self.timemap[key]) - 1
        while l <= r:
            m = l + (r - l)//2
            v,t = self.timemap[key][m]
            if t == timestamp:
                return v
            elif t < timestamp:
                l = m + 1
            else:
                r = m - 1
        return self.timemap[key][r][0] if r >=0 else ""
