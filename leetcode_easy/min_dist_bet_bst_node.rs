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
    let root_rc = Rc::new(RefCell::new(TreeNode::new(buffer[0].unwrap())));
    let mut q = VecDeque::new();
    q.push_back(Rc::clone(&root_rc));
    let mut curr = Rc::clone(&root_rc);
    let mut left = true;
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
    Some(root_rc)
}

// *** Solution ****

fn min_diff_in_bst_helper(
    root: &Option<Rc<RefCell<TreeNode>>>,
    diff: &mut i32,
    pred: &mut Option<i32>,
) {
    if let Some(node) = root {
        let _node = node.borrow();
        match _node.left {
            Some(ref subnode) => {
                min_diff_in_bst_helper(&Some(Rc::clone(subnode)), diff, pred);
            }
            None => {}
        };
        match pred {
            Some(x) => {
                *diff = i32::min(*diff, _node.val - *x);
            }
            None => {}
        }
        *pred = Some(_node.val);
        match _node.right {
            Some(ref subnode) => {
                min_diff_in_bst_helper(&Some(Rc::clone(subnode)), diff, pred);
            }
            None => {}
        };
    }
}

fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut diff = i32::MAX;
    let mut pred = None;
    min_diff_in_bst_helper(&root, &mut diff, &mut pred);
    diff
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let buffer = vec![Some(4), Some(2), Some(6), Some(1), Some(3)];
        let root = build_tree(buffer);
        assert_eq!(min_diff_in_bst(root), 1);
    }
    #[test]
    fn run_tc2() {
        let buffer = vec![Some(1), Some(0), Some(48), None, None, Some(12), Some(49)];
        let root = build_tree(buffer);
        assert_eq!(min_diff_in_bst(root), 1);
    }
    #[test]
    fn run_tc3() {
        let buffer = vec![Some(90), Some(69), None, Some(49), Some(89), None, Some(52)];
        let root = build_tree(buffer);
        assert_eq!(min_diff_in_bst(root), 1);
    }
}

fn main() {
    let buffer = vec![Some(4), Some(2), Some(6), Some(1), Some(3)];
    let root = build_tree(buffer);
    assert_eq!(min_diff_in_bst(root), 1);
}
