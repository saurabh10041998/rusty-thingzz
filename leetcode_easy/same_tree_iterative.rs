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
    let mut q = VecDeque::new();
    q.push_back(Rc::clone(&root_node));
    let mut is_left = true;
    for ele in buffer.iter().skip(1) {
        let new_node;
        match ele {
            Some(val) => {
                new_node = Some(Rc::new(RefCell::new(TreeNode::new(*val))));
                q.push_back(Rc::clone(new_node.as_ref().unwrap()));
            }
            None => {
                new_node = None;
            }
        }
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

fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut stack = vec![];
    match (p, q) {
        (Some(ref p_rc), Some(ref q_rc)) => {
            stack.push((Rc::clone(p_rc), Rc::clone(q_rc)));
        }
        (None, None) => {
            return true;
        }
        (_, _) => {
            return false;
        }
    };
    while let Some((p_node_rc, q_node_rc)) = stack.pop() {
        let _p_node = p_node_rc.borrow();
        let _q_node = q_node_rc.borrow();
        if _p_node.val != _q_node.val {
            return false;
        };
        match (_p_node.right.as_ref(), _q_node.right.as_ref()) {
            (Some(p_subnode), Some(q_subnode)) => {
                stack.push((Rc::clone(p_subnode), Rc::clone(q_subnode)));
            }
            (None, None) => {}
            (_, _) => {
                return false;
            }
        }
        match (_p_node.left.as_ref(), _q_node.left.as_ref()) {
            (Some(p_subnode), Some(q_subnode)) => {
                stack.push((Rc::clone(p_subnode), Rc::clone(q_subnode)));
            }
            (None, None) => {}
            (_, _) => {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let p = vec![Some(1), Some(2), Some(3)];
        let q = vec![Some(1), Some(2), Some(3)];
        let p_root = build_tree(p);
        let q_root = build_tree(q);
        assert_eq!(is_same_tree(p_root, q_root), true);
    }
    #[test]
    fn run_tc2() {
        let p = vec![Some(1), Some(2)];
        let q = vec![Some(1), None, Some(2)];
        let p_root = build_tree(p);
        let q_root = build_tree(q);
        assert_eq!(is_same_tree(p_root, q_root), false);
    }
    #[test]
    fn run_tc3() {
        let p = vec![Some(1), Some(2), Some(1)];
        let q = vec![Some(1), Some(1), Some(2)];
        let p_root = build_tree(p);
        let q_root = build_tree(q);
        assert_eq!(is_same_tree(p_root, q_root), false);
    }
}

fn main() {
    let p = vec![Some(1), Some(2), Some(3)];
    let q = vec![Some(1), Some(2), Some(3)];
    let p_root = build_tree(p);
    let q_root = build_tree(q);
    assert_eq!(is_same_tree(p_root, q_root), true);
}
