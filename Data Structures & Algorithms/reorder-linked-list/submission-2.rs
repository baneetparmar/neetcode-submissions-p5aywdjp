impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut len = 0;
        let mut cur = head.as_deref();
        while let Some(node) = cur {
            len += 1;
            cur = node.next.as_deref();
        }
        if len <= 2 { return; }

        let mut cur = head.as_mut();
        for _ in 0..len / 2 {
            cur = cur.unwrap().next.as_mut();
        }
        // `cur` now points at the Option<Box<...>> that is the start of
        // the second half. Taking it gives us ownership cleanly.
        let mut second = cur.unwrap().next.take();

        let mut prev: Option<Box<ListNode>> = None;
        while let Some(mut node) = second {
            second = node.next.take();
            node.next = prev;         
            prev = Some(node);      
        }

        let mut first = head;
        let mut second = prev;
        while second.is_some() {
            let first_next = first.as_mut().unwrap().next.take();
            let second_next = second.as_mut().unwrap().next.take();

            first.as_mut().unwrap().next = second; 
            first = &mut first.as_mut().unwrap().next; 
            first.as_mut().unwrap().next = first_next;
            first = &mut first.as_mut().unwrap().next;

            second = second_next;
        }
    }
}