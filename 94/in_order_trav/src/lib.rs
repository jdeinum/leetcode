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

// go left
// process node
// go right
pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut acc: Vec<i32> = Vec::new();
    return inorder_traversal_rec(root, &mut acc);
}

pub fn inorder_traversal_rec(root: Option<Rc<RefCell<TreeNode>>>, acc: &mut Vec<i32>) -> Vec<i32> {
    match root {
        None => return acc.to_vec(),
        Some(r) => {
            inorder_traversal_rec(r.borrow().left.clone(), acc);
            acc.push(r.borrow().val);
            inorder_traversal_rec(r.borrow().right.clone(), acc);
        }
    }
    return acc.to_vec();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
