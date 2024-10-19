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

type Node = Rc<RefCell<TreeNode>>;
type Link = Option<Node>;
type Children = Vec<Link>;

impl TreeNode {
    pub fn get_left(&self) -> Link {
        return self.left.clone();
    }

    pub fn get_right(&self) -> Link {
        return self.right.clone();
    }

    pub fn get_value(&self) -> i32 {
        self.clone().borrow().val
    }

    pub fn get_children(&self) -> Children {
        vec![self.get_left(), self.get_right()]
    }
}

pub fn make_move(current: &mut Vec<i32>, value: Option<TreeNode>) {
    match value {
        None => {}
        Some(a) => current.push(a.get_value()),
    }
}

pub fn unmake_move(current: &mut Vec<i32>) {
    current.pop();
}

pub fn path_sum_rec(
    current_node: Node,
    target_sum: i32,
    current: &mut Vec<i32>,
    acc: &mut Vec<Vec<i32>>,
) {
    if current.iter().sum::<i32>() == target_sum {
        acc.push(current.clone())
    }

    let x: &TreeNode = current_node.clone().borrow();

    for node in current_node.clone().borrow().get_children() {
        make_move(current, node.clone());
        path_sum_rec(node, target_sum, current, acc);
        unmake_move(current);
    }
}

pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
    return match root {
        None => vec![],
        Some(x) => {
            let mut acc: Vec<Vec<i32>> = Vec::new();
            let mut current: Vec<i32> = vec![];
            current.push(x.borrow().val);
            path_sum_rec(Some(x), target_sum, &mut current, &mut acc);
            acc
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {}
}
