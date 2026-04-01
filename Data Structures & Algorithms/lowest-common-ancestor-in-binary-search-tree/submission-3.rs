use std::rc::Rc;
use std::cell::RefCell;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        
        let mut head = root.clone();
        let p = p.unwrap().borrow().val;
        let q = q.unwrap().borrow().val;

        while let Some(node) = head {
            let val = node.borrow().val;


            if val > p && val > q {
                head = node.borrow().left.clone();
            } else if val < p && val < q {
                head = node.borrow().right.clone();
            } else {
                return Some(node.clone());
            }
        }
        None
    }
}
