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

pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
    let mut acc: Vec<i32> = Vec::new();
    let mut it: &mut Option<Box<ListNode>> = &mut head.clone();
    while let Some(a) = it {
        acc.push(a.val);
        it = &mut a.next;
    }

    let mut result = 0i32;
    println!("{:?}", acc);
    for (i, &bit) in acc.iter().rev().enumerate() {
        if bit == 1 {
            result |= 1 << i;
        }
    }
    result
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {}
}
