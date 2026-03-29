# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def findPath(self,root,key):
        path = []
        while True:
            path.append(root)
            if key == root.val:
                return path
            elif root.val < key:
                root = root.right
            # root.val > key:
            else:
                root = root.left

    def lowestCommonAncestor(self, root: TreeNode, p: TreeNode, q: TreeNode) -> TreeNode:
        path1 = self.findPath(root,p.val)
        path2 = self.findPath(root,q.val)

        res = root
        for i in range(min(len(path1),len(path2))):
            if path1[i] != path2[i]:
                break
            res = path1[i]
        return res