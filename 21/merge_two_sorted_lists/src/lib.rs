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

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list1, list2) {
        (Some(a), Some(b)) if a.val > b.val => {
            let mut x = Box::new(ListNode::new(b.val));
            x.next = Solution::merge_two_lists(Some(a), b.next);
            return Some(x);
        }
        (Some(a), Some(b)) => {
            let mut x = Box::new(ListNode::new(a.val));
            x.next = Solution::merge_two_lists(a.next, Some(b));
            return Some(x);
        }
        (Some(a), None) => {
            let mut x = Box::new(ListNode::new(a.val));
            x.next = Solution::merge_two_lists(a.next, None);
            return Some(x);
        }
        (None, Some(b)) => {
            let mut x = Box::new(ListNode::new(b.val));
            x.next = Solution::merge_two_lists(None, b.next);
            return Some(x);
        }
        (None, None) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
