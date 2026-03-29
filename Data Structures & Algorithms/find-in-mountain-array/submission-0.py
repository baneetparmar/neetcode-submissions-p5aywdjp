class Solution:
    def findInMountainArray(self, target: int, mountainArr: 'MountainArray') -> int:
        get = mountainArr.get
        l = 0
        r = mountainArr.length() - 1
        # search for peak
        while l <= r:
            m = l + (r - l)//2
            # left < right -> increasing -> peak on right -> move right
            if get(m) < get(m + 1):
                l = m + 1
            else:
                # decreasing -> peak on left -> move left
                r = m - 1
        # peak is at l
        peak = l

        # search: start -> peak
        l = 0
        r = peak
        while l <= r:
            m = l + (r - l)//2
            val = get(m)
            if val == target:
                return m
            elif val < target:
                l = m + 1
            else:
                r = m - 1
        
        # search: peak -> end
        l = peak
        r = mountainArr.length() - 1
        while l <= r:
            m = l + (r - l)//2
            val = get(m)
            if val == target:
                return m
            elif val < target:
                r = m - 1
            else:
                l = m + 1

        return -1