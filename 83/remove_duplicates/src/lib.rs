use std::collections::HashSet;

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

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }
    let mut h = head;
    let mut previous = h.as_mut().unwrap();
    while let Some(current) = previous.next.as_mut() {
        if current.val == previous.val {
            previous.next = current.next.take();
        } else {
            previous = previous.next.as_mut().unwrap();
        }
    }
    h
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
