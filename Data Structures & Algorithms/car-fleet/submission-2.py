class Solution:
    def carFleet(self, target: int, position: List[int], speed: List[int]) -> int:
        cars = [(position[i],speed[i]) for i in range(len(position))]
        cars.sort(key=lambda x: x[0],reverse=True)
        fleet = 0
        last_fleet_time = 0
        for p,s in cars:
            t = float(target - p)/s
            if t > last_fleet_time:
                fleet += 1
                last_fleet_time = t
            
        return fleet