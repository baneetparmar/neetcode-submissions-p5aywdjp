use std::rc::Rc;
use std::cell::RefCell;

type T = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn is_valid_bst(root: T) -> bool {
        fn dfs(node: T, min: i64, max: i64) -> bool {
            if let Some(n) = node {
                let val = n.borrow().val as i64;
                let left = n.borrow().left.clone();
                let right = n.borrow().right.clone();

                val > min && val < max
                    && dfs(left, min, val)
                    && dfs(right, val, max)
            } else {
                true
            }
        }

        dfs(root, i64::MIN, i64::MAX)
    }
}
