use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    return match (p, q) {
        (None, None) => true,
        (Some(_), None) => false,
        (None, Some(_)) => false,
        (Some(a), Some(b)) if a.borrow().val != b.borrow().val => false,
        (Some(a), Some(b)) => {
            if !is_same_tree(a.borrow().left, b.borrow().left) {
                return false;
            }
            if !is_same_tree(a.borrow().right, b.borrow().right) {
                return false;
            }

            true
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
