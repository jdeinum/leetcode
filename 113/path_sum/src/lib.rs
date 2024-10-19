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

// making a move consists of adding the node to the current list of nodes
// Making a move consists of adding the node's value to the current list of nodes
pub fn make_move(current: &mut Vec<i32>, value: Option<Rc<RefCell<TreeNode>>>) {
    if let Some(node) = value {
        current.push(node.clone().borrow_mut().val); // Push the value of the node
    }
}

pub fn unmake_move(current: &mut Vec<i32>) {
    current.pop();
}

pub fn get_children(current: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    return match current {
        None => vec![],
        Some(a) => {
            let mut res = vec![];
            res.push(a.clone().borrow_mut().left.clone());
            res.push(a.clone().borrow_mut().right.clone());
            res
        }
    };
}

pub fn path_sum_rec(
    current_node: Option<Rc<RefCell<TreeNode>>>,
    target_sum: i32,
    current: &mut Vec<i32>,
    acc: &mut Vec<Vec<i32>>,
) {
    // we found a path!
    if current.iter().sum::<i32>() == target_sum
        && get_children(current_node.clone())
            .iter()
            .filter(|x| x.is_some())
            .count()
            > 0
    {
        acc.push(current.clone())
    }

    if current_node.is_none() {
        return;
    }

    let children: Vec<Option<Rc<RefCell<TreeNode>>>> = get_children(current_node);
    for node in children {
        if node.is_none() {
            continue;
        }
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
            current.push(x.borrow_mut().val);
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
