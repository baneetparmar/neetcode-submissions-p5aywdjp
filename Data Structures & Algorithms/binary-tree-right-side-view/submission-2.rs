use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut q = VecDeque::new();
        let mut res = vec![];
        q.push_back(root);

        loop {
            let mut mid_res = vec![];
            let level_size = q.len();

            for _ in 0..level_size {
                if let Some(Some(node)) = q.pop_front() {
                    let n = node.borrow();
                    mid_res.push(n.val);
                    q.push_back(n.left.clone());
                    q.push_back(n.right.clone());
                }
            }

            if mid_res.is_empty() { break; }
            res.push(mid_res.pop().unwrap());
        }
        res
    }
}
