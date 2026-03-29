class Solution:
    INF = 10 ** 10

    def findClosestElements(self, arr: List[int], k: int, x: int) -> List[int]:

        # Step 1: find closest index
        start = 0
        dist = self.INF
        for i in range(len(arr)):
            if abs(arr[i] - x) < dist:
                dist = abs(arr[i] - x)
                start = i

        # Step 2: expand window
        left = start
        right = start

        while (right - left + 1) < k:
            if left == 0:
                right += 1
            elif right == len(arr) - 1:
                left -= 1
            else:
                # left priority on tie
                if abs(arr[left - 1] - x) <= abs(arr[right + 1] - x):
                    left -= 1
                else:
                    right += 1

        return arr[left:right + 1]
