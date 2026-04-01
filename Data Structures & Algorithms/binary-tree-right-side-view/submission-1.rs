// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Option<Rc<RefCell<TreeNode>>>,
//     pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         TreeNode {
//             val,
//             left: None,
//             right: None,
//         }
//     }
// }

use std::rc::Rc;
use std::cell::RefCell;

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
