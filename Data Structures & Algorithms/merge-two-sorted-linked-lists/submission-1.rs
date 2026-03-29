impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        while let (Some(l1), Some(l2)) = (list1.as_ref(), list2.as_ref()) {
            if l1.val < l2.val {
                tail.next = list1;
                tail = tail.next.as_mut().unwrap();
                list1 = tail.next.take();
            } else {
                tail.next = list2;
                tail = tail.next.as_mut().unwrap();
                list2 = tail.next.take();
            }
        }

        tail.next = if list1.is_some() { list1 } else { list2 };
        dummy.next
    }
}