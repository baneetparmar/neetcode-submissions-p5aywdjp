use std::cell::RefCell;
use std::rc::Rc;
type T = Option<Rc<RefCell<TreeNode>>>;

impl Solution {
    pub fn max_path_sum(root: T) -> i32 {
        fn dfs(node: T, res: &mut i32) -> i32 {
            if let Some(n) = node {
                let left = dfs(n.borrow().left.clone(), res).max(0);
                let right = dfs(n.borrow().right.clone(), res).max(0);
                let val = n.borrow().val;

                *res = (*res).max(val + left + right);

                val + left.max(right)
            } else {
                0
            }
        }

        let mut res = i32::MIN;
        dfs(root, &mut res);
        res
    }
}