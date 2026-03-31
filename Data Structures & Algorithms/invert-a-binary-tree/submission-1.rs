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
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let head = root.clone();

        fn reverse(root: Option<Rc<RefCell<TreeNode>>>) {
            if let Some(node) = root {
                (node.borrow_mut().left, node.borrow_mut().right) = (node.borrow().right.clone(), node.borrow().left.clone());
                reverse(node.borrow().right.clone());
                reverse(node.borrow().left.clone());
            }
        }
        reverse(root);
        head
    }
}
