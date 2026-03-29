// Definition for a Node.
// #[derive(Clone, Debug, PartialEq, Eq)]
// pub struct Node {
//     pub val: i32,
//     pub next: Option<Rc<RefCell<Node>>>,
//     pub random: Option<Rc<RefCell<Node>>>,
// }
//
// impl Node {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         Node {
//             val,
//             next: None,
//             random: None,
//         }
//     }
// }

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

impl Solution {
    pub fn copy_random_list(
        head: Option<Rc<RefCell<Node>>>
    ) -> Option<Rc<RefCell<Node>>> {

        let mut map: HashMap<*const RefCell<Node>, Rc<RefCell<Node>>> = HashMap::new();

        let mut curr = head.clone();
        while let Some(node) = curr {
            let node_ref = node.borrow();
            let new_node = Rc::new(RefCell::new(Node::new(node_ref.val)));

            map.insert(Rc::as_ptr(&node), new_node);

            curr = node_ref.next.clone();
        }

        let mut curr = head.clone();
        while let Some(node) = curr {
            let node_ref = node.borrow();
            let new_node = map.get(&Rc::as_ptr(&node)).unwrap();

            if let Some(next) = node_ref.next.clone() {
                let new_next = map.get(&Rc::as_ptr(&next)).unwrap().clone();
                new_node.borrow_mut().next = Some(new_next);
            }

            if let Some(random) = node_ref.random.clone() {
                let new_random = map.get(&Rc::as_ptr(&random)).unwrap().clone();
                new_node.borrow_mut().random = Some(new_random);
            }

            curr = node_ref.next.clone();
        }

        head.map(|node| map.get(&Rc::as_ptr(&node)).unwrap().clone())
    }
}