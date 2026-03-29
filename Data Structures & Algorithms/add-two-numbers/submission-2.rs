impl Solution {
  pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
      let mut dummy = Box::new(ListNode{val:0,next:None});
      let mut tail = &mut dummy;

      let mut l1 = l1;
      let mut l2 = l2;
      let mut carry = 0;
      while (l1.is_some() || l2.is_some() || carry > 0) {
          let dig1 = l1.as_deref().map_or(0, |n| n.val);
          let dig2 = l2.as_deref().map_or(0, |n| n.val);
          let sum = dig1 + dig2 + carry;
          carry = sum / 10;

          tail.next = Some(Box::new(ListNode::new(sum % 10)));
          tail = tail.next.as_mut().unwrap();

          l1 = l1.and_then(|n| n.next);
          l2 = l2.and_then(|n| n.next);
      }
      dummy.next
  }
}
