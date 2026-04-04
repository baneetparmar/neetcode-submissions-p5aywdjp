use std::cell::RefCell;
use std::rc::Rc;
type T = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn remove_leaf_nodes(root: T, target: i32) -> T {
        if let Some(node) = root {
            {
                let mut n = node.borrow_mut();
                n.left = Self::remove_leaf_nodes(n.left.clone(), target);
                n.right = Self::remove_leaf_nodes(n.right.clone(), target);
            }

            let n = node.borrow();
            if n.val == target && n.left.is_none() && n.right.is_none() {
                None
            } else {
                Some(node.clone())
            }
        } else {
            None
        }
    }
}

