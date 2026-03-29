# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def bfs(self,root):
        queue = deque()
        queue.append([root])
        res = []
        while queue:
            vals = []
            temp = []
            for node in queue.popleft():
                if node:
                    vals.append(node.val)
                    temp.append(node.left)
                    temp.append(node.right)
            if vals:
                res.append(vals.pop())
            if temp:
                queue.append(temp)
        return res

    def rightSideView(self, root: Optional[TreeNode]) -> List[int]:
        return self.bfs(root)