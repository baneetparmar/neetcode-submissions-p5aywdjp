class Solution:
    def shipWithinDays(self, weights: List[int], days: int) -> int:
        l = max(weights)
        r = sum(weights)
        min_cap = r
        while l <= r:
            load = 0
            loaded = 0
            m = l + (r - l)//2 # choosen capacity
            for n in weights:
                load += n
                if load == m: # load = capacity
                    loaded += 1
                    load = 0
                elif load > m:
                    load = n
                    loaded += 1
            if load > 0:
                loaded += 1
            if loaded <= days: # shipment in time -> can lower capcity -> move left
                min_cap = min(min_cap,m)
                r = m - 1
            else:
                #shippment overdue -> increase capacity -> move right
                l = m + 1
        return min_cap