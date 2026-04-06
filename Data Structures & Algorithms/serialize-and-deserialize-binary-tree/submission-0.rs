use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
type T = Option<Rc<RefCell<TreeNode>>>;


struct Codec {}

impl Codec {
    fn new() -> Self {
        Self {}
    }

    fn serialize(&self, root: T) -> String {
        if root.is_none(){
            return "".to_string()
        }
        let mut v = vec![];
        let mut deq = VecDeque::new();
        deq.push_back(root);

        while let Some(node) = deq.pop_front() {
            if let Some(n) = node {
                let val = n.borrow().val.clone();
                v.push(format!("{}", val));
                deq.push_back(n.borrow().left.clone());
                deq.push_back(n.borrow().right.clone());
            } else {
                v.push("nil".to_string());
            }
        }
        v.join(",")
    }

    fn deserialize(&self, data: String) -> T {
        if data.is_empty() {
            return None;
        }
        let mut vals = data.split(",");

        let root_val = vals.next().unwrap().parse::<i32>().unwrap();
        let mut root = Rc::new(RefCell::new(TreeNode::new(root_val)));
        let mut deq = VecDeque::new();
        deq.push_back(root.clone());

        while let Some(node) = deq.pop_front() {
            // assign left child
            if let Some(val) = vals.next() {
                if val != "nil" {
                    let left = Rc::new(RefCell::new(TreeNode::new(val.parse::<i32>().unwrap())));
                    node.borrow_mut().left = Some(left.clone());
                    deq.push_back(left);
                }
            }

            // assign right child
            if let Some(val) = vals.next() {
                if val != "nil" {
                    let right = Rc::new(RefCell::new(TreeNode::new(val.parse::<i32>().unwrap())));
                    node.borrow_mut().right = Some(right.clone());
                    deq.push_back(right);
                }
            }
        }
        Some(root)
    }
}