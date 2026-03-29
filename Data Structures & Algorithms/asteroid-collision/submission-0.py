class Solution:
    def asteroidCollision(self, asteroids: List[int]) -> List[int]:
        res = []
        i = 0
        while i < len(asteroids):
            if not res or asteroids[i] >= 0:
                res.append(asteroids[i])
            else:
                live = True
                while res and live:
                    if res[-1] == abs(asteroids[i]): # equals
                        res.pop()
                        live = False
                    elif res[-1] < 0: # moving in the same direction
                        live = True
                        break
                    elif  res[-1] >= 0 and res[-1] < abs(asteroids[i]): # asteroids comming are weaker
                        res.pop()
                        live = True
                    else: # greater
                        live = False
                if live:
                    res.append(asteroids[i])
            i += 1
        return res