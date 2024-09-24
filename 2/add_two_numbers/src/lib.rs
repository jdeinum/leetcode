#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    return add_two_numbers_rec(l1, l2, 0);
}

pub fn add_two_numbers_rec(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
    carry: i32,
) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (Some(a), Some(b)) => {
            let total = a.val + b.val + carry;
            let mut next_node = Box::new(ListNode::new(total % 10));
            next_node.next = add_two_numbers_rec(a.next, b.next, total / 10);
            return Some(next_node);
        }
        (Some(a), None) => {
            let total = a.val + carry;
            let mut next_node = Box::new(ListNode::new(total % 10));
            next_node.next = add_two_numbers_rec(a.next, None, total / 10);
            return Some(next_node);
        }
        (None, Some(b)) => {
            let total = b.val + carry;
            let mut next_node = Box::new(ListNode::new(total % 10));
            next_node.next = add_two_numbers_rec(None, b.next, total / 10);
            return Some(next_node);
        }
        (None, None) => {
            if carry != 0 {
                let mut next_node = Box::new(ListNode::new(1));
                next_node.next = None;
                return Some(next_node);
            }
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
