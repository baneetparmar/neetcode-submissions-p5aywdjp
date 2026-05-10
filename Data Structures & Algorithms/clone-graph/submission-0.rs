use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

impl Solution {
    pub fn clone_graph(node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        let mut visited: HashMap<i32, Rc<RefCell<Node>>> = HashMap::new();
        Self::dfs(node, &mut visited)
    }

    fn dfs(
        node: Option<Rc<RefCell<Node>>>,
        visited: &mut HashMap<i32, Rc<RefCell<Node>>>,
    ) -> Option<Rc<RefCell<Node>>> {
        let node = node?;
        let val = node.borrow().val;
        if let Some(clone) = visited.get(&val) {
            return Some(clone.clone());
        }
        let clone = Rc::new(RefCell::new(Node::new(val)));
        visited.insert(val, clone.clone());
        for neighbor in &node.borrow().neighbors {
            let n = Self::dfs(Some(neighbor.clone()), visited);
            clone.borrow_mut().neighbors.push(n.unwrap());
        }
        Some(clone)
    }
}