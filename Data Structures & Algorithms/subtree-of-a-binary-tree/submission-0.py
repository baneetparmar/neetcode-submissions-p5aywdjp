# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right

class Solution:
    def sameTree(self,p,q):
        que = deque()
        que.append((p,q))

        while que:
            node_p, node_q = que.popleft()
            
            if (node_p is None) != (node_q is None):
                return False
            
            if (node_p and node_q) and (node_p.val != node_q.val):
                return False
            if node_p:
                que.append((node_p.left ,node_q.left ))
                que.append((node_p.right,node_q.right))
        return True

    def isSubtree(self, root: Optional[TreeNode], subRoot: Optional[TreeNode]) -> bool:
        if not subRoot: return True
        if not root: return False

        if self.sameTree(root,subRoot):
            return True
        return (self.isSubtree(root.left,subRoot) or self.isSubtree(root.right,subRoot))