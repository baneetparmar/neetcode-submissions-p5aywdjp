class Solution:
    def trap(self, height: List[int]) -> int:
        water = 0
        base = 1
        # find second height line
        m = 0
        sm = 0
        for n in height:
            if n > sm:
                sm = n
            if n > m:
                sm = m
                m = n
            

        for _ in range(sm):
            l = 0
            r = len(height) - 1

            # find left edge to container
            while l < len(height) - 1 and height[l] < base:
                l += 1
            # find right edge to container
            while r > 0 and height[r] < base:
                r -= 1
            for n in height[l + 1: r]:
                if n < base:
                    water += 1

            base += 1
        return water
            
                    
            