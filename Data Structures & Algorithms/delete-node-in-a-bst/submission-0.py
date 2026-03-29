class Solution:
    def deleteNode(self, root: Optional[TreeNode], key: int) -> Optional[TreeNode]:
        if not root:
            return None

        if key < root.val:
            root.left = self.deleteNode(root.left, key)
        elif key > root.val:
            root.right = self.deleteNode(root.right, key)
        else:
            # Case 1: No left child
            if not root.left:
                return root.right

            # Case 2: No right child
            if not root.right:
                return root.left

            # Case 3: Two children
            # Find inorder predecessor (max in left subtree)
            temp = root.left
            while temp.right:
                temp = temp.right

            root.val = temp.val
            root.left = self.deleteNode(root.left, temp.val)

        return root