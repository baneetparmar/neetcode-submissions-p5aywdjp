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

pub fn reverse_between(head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode {val:-1, next:head});
    let mut prev = &mut dummy;

    for _ in 0..(left - 1){
        prev = prev.next.as_mut().unwrap();
    }

    let mut curr = prev.next.take();
    let mut reversed:Option<Box<ListNode>> = None;
    for _ in 0..(right - left + 1) {
        let mut node = curr.unwrap();
        curr = node.next.take();
        node.next = reversed;
        reversed = Some(node);
    }

    let mut tail = &mut reversed;
    while tail.as_deref().unwrap().next.is_some() {
        tail = &mut tail.as_mut().unwrap().next;
    }
    tail.as_mut().unwrap().next = curr;
    prev.next = reversed;

    dummy.next
}
}
