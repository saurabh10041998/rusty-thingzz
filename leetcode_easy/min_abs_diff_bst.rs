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
    if buffer.is_empty() {
        return None;
    }
    let root_rc = Rc::new(RefCell::new(TreeNode::new(buffer[0].unwrap())));
    let mut q = VecDeque::new();
    q.push_back(Rc::clone(&root_rc));
    let mut curr = Rc::clone(&root_rc);
    let mut left = true;
    for ele in buffer.into_iter().skip(1) {
        let new_node;
        match ele {
            Some(val) => {
                new_node = Some(Rc::new(RefCell::new(TreeNode::new(val))));
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
    Some(root_rc)
}

fn helper(root: &Option<Rc<RefCell<TreeNode>>>, lst: &mut Vec<i32>) {
    if let Some(node) = root {
        let _node = node.borrow();
        match _node.left {
            Some(ref subnode) => {
                helper(&Some(Rc::clone(subnode)), lst);
            }
            None => {}
        }
        lst.push(_node.val);
        match _node.right {
            Some(ref subnode) => {
                helper(&Some(Rc::clone(subnode)), lst);
            }
            None => {}
        }
    }
}

fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut lst = vec![];
    helper(&root, &mut lst);
    lst.windows(2).map(|x| x[1] - x[0]).min().unwrap()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let buffer = vec![Some(4), Some(2), Some(6), Some(1), Some(3)];
        let root = build_tree(buffer);
        assert_eq!(get_minimum_difference(root), 1);
    }
    #[test]
    fn run_tc2() {
        let buffer = vec![Some(1), Some(0), Some(48), None, None, Some(12), Some(49)];
        let root = build_tree(buffer);
        assert_eq!(get_minimum_difference(root), 1);
    }
}

fn main() {
    let buffer = vec![Some(1), Some(0), Some(48), None, None, Some(12), Some(49)];
    let root = build_tree(buffer);
    assert_eq!(get_minimum_difference(root), 1);
}
