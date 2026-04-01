use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match node {
                None => 0,
                Some(n) => {
                    let left = n.borrow().left.clone();
                    let right = n.borrow().right.clone();

                    let left_height =  dfs(left);
                    let right_height = dfs(right);
                    if left_height == -1 || right_height == -1 || (left_height - right_height).abs() > 1 {
                        return -1 
                    }
                    1 + left_height.max(right_height)
                }
            }
        }
        !(dfs(root) == -1)
        
    }
}
