use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn build_tree(buffer: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if buffer.len() == 0 {
        return None;
    }
    let root_node = Rc::new(RefCell::new(TreeNode::new(buffer[0].unwrap())));
    let mut curr = Rc::clone(&root_node);
    let mut is_left = true;
    let mut q = VecDeque::new();
    q.push_back(Rc::clone(&root_node));
    for ele in buffer.iter().skip(1) {
        let new_node;
        match ele {
            Some(v) => {
                new_node = Some(Rc::new(RefCell::new(TreeNode::new(*v))));
                q.push_back(Rc::clone(new_node.as_ref().unwrap()));
            }
            None => {
                new_node = None;
            }
        };
        if is_left {
            curr = q.pop_front().unwrap();
            curr.borrow_mut().left = new_node;
            is_left = false;
        } else {
            curr.borrow_mut().right = new_node;
            is_left = true;
        }
    }
    Some(root_node)
}

fn max_depth_inner(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(ref node) = root {
        let _node = node.borrow();
        let result1 = match _node.left {
            Some(ref subnode) => max_depth_inner(Some(Rc::clone(subnode))),
            None => 0,
        };
        let result2 = match _node.right {
            Some(ref subnode) => max_depth_inner(Some(Rc::clone(subnode))),
            None => 0,
        };
        return i32::max(result1, result2) + 1;
    }
    0
}

fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    max_depth_inner(root)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let buffer = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        let root = build_tree(buffer);
        assert_eq!(max_depth(root), 3);
    }

    #[test]
    fn run_tc2() {
        let buffer = vec![Some(1), None, Some(2)];
        let root = build_tree(buffer);
        assert_eq!(max_depth(root), 2);
    }
}

fn main() {
    let buffer = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
    let root = build_tree(buffer);
    assert_eq!(max_depth(root), 3);
}
