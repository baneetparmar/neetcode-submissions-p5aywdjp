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

    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode { val:0, next:head });

        let steps = {
            let mut fast = dummy.as_ref();
            let mut len = 0;
            while fast.next.is_some() {
                fast = fast.next.as_deref().unwrap();
                len += 1;
            }
            len
        };

        let mut slow = dummy.as_mut();
        for _ in 0..(steps - n as usize){
            slow = slow.next.as_deref_mut().unwrap();
        }

        slow.next = slow.next.as_deref_mut().unwrap().next.take();
        
        dummy.next
    }

}
