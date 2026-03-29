# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def invertTree(self, root: Optional[TreeNode]) -> Optional[TreeNode]:
        head = root
        
        def reverse(node):
            if not node:
                return
            node.left, node.right = node.right,node.left
            reverse(node.right)
            reverse(node.left)
        reverse(root)
        return head