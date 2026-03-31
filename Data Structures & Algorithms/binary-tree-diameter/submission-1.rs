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
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut res = 0;

        fn dfs(node: Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> i32 {
            match node {
                None => 0,
                Some(n) => {
                    let left = n.borrow().left.clone();
                    let right = n.borrow().right.clone();

                    let left_depth = dfs(left, res);
                    let right_depth = dfs(right, res);

                    *res = (*res).max(left_depth + right_depth);
                    1 + left_depth.max(right_depth)
                }
            }
        }

        dfs(root, &mut res);
        res
    }
}
