use std::rc::Rc;
use std::cell::RefCell;

type T = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn rob(root: T) -> i32 {
        fn dfs(n: &T) -> i32 {
            if let Some(node) = n {
                let node = node.borrow();

                let mut res = node.val;

                if let Some(ref left) = node.left {
                    let left = left.borrow();
                    res += dfs(&left.left) + dfs(&left.right);
                };
                if let Some(ref right) = node.right {
                    let right = right.borrow();
                    res += dfs(&right.left) + dfs(&right.right);
                };

                res.max(dfs(&node.left) + dfs(&node.right))
            } else {
                0
            }
        }
        dfs(&root)
    }
}
