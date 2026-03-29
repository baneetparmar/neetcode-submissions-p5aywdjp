class Solution:
    def carFleet(self, target: int, position: List[int], speed: List[int]) -> int:
        cars = [(position[i],speed[i]) for i in range(len(position))]
        cars.sort(key=lambda x: x[0],reverse=True)
        res = []
        for p,s in cars:
            t = float(target - p)/s
            if res and t <= res[-1]:
                continue
            res.append(t)
        return len(res)