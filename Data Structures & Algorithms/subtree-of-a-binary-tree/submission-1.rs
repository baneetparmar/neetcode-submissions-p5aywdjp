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
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut is_same = true;
        fn dfs(
            t1: Option<Rc<RefCell<TreeNode>>>,
            t2: Option<Rc<RefCell<TreeNode>>>,
            is_same: &mut bool,
        ) {
            match (t1, t2) {
                (Some(p), Some(q)) => {
                    let p = p.borrow();
                    let q = q.borrow();

                    if p.val != q.val {
                        *is_same = false;
                        return;
                    }

                    dfs(p.left.clone(), q.left.clone(), is_same);
                    dfs(p.right.clone(), q.right.clone(), is_same)
                }

                (None, None) => {}
                _ => {
                    *is_same = false;
                }
            }
        }
        dfs(p, q, &mut is_same);
        is_same
    }

    pub fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => false,
            Some(node) => {
                let left = node.borrow().left.clone();
                let right = node.borrow().right.clone();

                Self::is_same_tree(Some(node), sub_root.clone())
                    || Self::is_subtree(left, sub_root.clone())
                    || Self::is_subtree(right, sub_root)
            }
        }
    }
}
