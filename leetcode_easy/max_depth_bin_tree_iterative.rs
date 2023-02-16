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
    let root = Rc::new(RefCell::new(TreeNode::new(buffer[0].unwrap())));
    let mut q = VecDeque::new();
    q.push_back(Rc::clone(&root));
    let mut left = true;
    let mut curr = Rc::clone(&root);
    for c in buffer.iter().skip(1) {
        let new_node;
        match c {
            Some(x) => {
                new_node = Some(Rc::new(RefCell::new(TreeNode::new(*x))));
                q.push_back(Rc::clone(new_node.as_ref().unwrap()));
            }
            None => {
                new_node = None;
            }
        }
        if left {
            curr = q.pop_front().unwrap();
            curr.borrow_mut().left = new_node;
            left = false;
        } else {
            curr.borrow_mut().right = new_node;
            left = true;
        }
    }
    Some(root)
}

fn max_depth_iterative(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = root {
        let mut q = VecDeque::new();
        q.push_back(Rc::clone(node));
        let mut count = 0;
        while !q.is_empty() {
            let mut len = q.len();
            while len > 0 {
                let curr = q.pop_front().unwrap();
                let _curr = curr.borrow();
                match _curr.left {
                    Some(ref subnode) => q.push_back(Rc::clone(subnode)),
                    None => {}
                }
                match _curr.right {
                    Some(ref subnode) => q.push_back(Rc::clone(subnode)),
                    None => {}
                }
                len -= 1;
            }
            count += 1;
        }
        return count;
    }
    0
}

fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    max_depth_iterative(&root)
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
