impl Solution {
    fn list_to_reverse_vector(mut list:Option<Box<ListNode>>) -> Vec<i32>{
        let mut nodes = vec![];
        while let Some(node) = list {
            nodes.push(node.val);
            list = node.next;
        }
        nodes
    }

    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let l1 = Self::list_to_reverse_vector(l1);
        let l2 = Self::list_to_reverse_vector(l2);

        let max_len = l1.len().max(l2.len());
        let mut carry = 0;
        let mut sums = Vec::with_capacity(max_len + 1);

        for i in 0..max_len {
            let a = l1.get(i).copied().unwrap_or(0);
            let b = l2.get(i).copied().unwrap_or(0);
            let sum = a + b + carry;
            carry = sum / 10;
            sums.push(sum % 10);
        }
        if carry > 0 {
            sums.push(carry);
        }

        let mut head: Option<Box<ListNode>> = None;
        for val in sums.into_iter().rev() {
            head = Some(Box::new(ListNode { val, next: head }));
        }

        head
    }
}
