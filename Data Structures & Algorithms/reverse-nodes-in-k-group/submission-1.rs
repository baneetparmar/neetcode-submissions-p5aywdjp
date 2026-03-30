// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

impl Solution {
pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    dummy.next = head;
    let mut group_prev = &mut dummy;

    loop {
        let mut check = group_prev.next.as_deref();
        for _ in 0..k {
            match check {
                None => return dummy.next,
                Some(node) => {
                    check = node.next.as_deref();
                }
            }
        }

        let mut curr = group_prev.next.take();
        let mut reversed: Option<Box<ListNode>> = None;
        let mut tail: *mut ListNode = std::ptr::null_mut();

        for i in 0..k {
            let mut node = curr.unwrap();
            curr = node.next.take();
            if i == 0 {
                tail = &mut *node as *mut ListNode;
            }
            node.next = reversed;
            reversed = Some(node);
        }

        unsafe { (*tail).next = curr; }
        group_prev.next = reversed;

        for _ in 0..k {
            group_prev = group_prev.next.as_mut().unwrap();
        }
    }
}
}
