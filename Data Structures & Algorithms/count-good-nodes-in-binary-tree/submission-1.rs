use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>,  res: &mut i32, max: i32,) {
            if let Some(n) = node {
                let val = n.borrow().val;
                if val >= max { *res += 1; }
                let left = n.borrow().left.clone();
                let right = n.borrow().right.clone();
                dfs(left, res, max.max(val),);
                dfs(right, res, max.max(val),);
            }
        }

        let mut res = 0;
        let max = root.as_ref().unwrap().borrow().val;
        dfs(root, &mut res, max);
        res
    }
}
