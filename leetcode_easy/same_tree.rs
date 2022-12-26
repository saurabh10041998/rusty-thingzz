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

// Solution..
fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (Some(ref p_node_rc), Some(ref q_node_rc)) => {
            let p_node = p_node_rc.borrow();
            let q_node = q_node_rc.borrow();
            if p_node.val != q_node.val {
                return false;
            }
            let result1 = match (p_node.left.as_ref(), q_node.left.as_ref()) {
                (Some(ref subnode1), Some(ref subnode2)) => {
                    is_same_tree(Some(Rc::clone(subnode1)), Some(Rc::clone(subnode2)))
                }
                (None, None) => true,
                (_, _) => false,
            };
            let result2 = match (p_node.right.as_ref(), q_node.right.as_ref()) {
                (Some(ref subnode1), Some(ref subnode2)) => {
                    is_same_tree(Some(Rc::clone(subnode1)), Some(Rc::clone(subnode2)))
                }
                (None, None) => true,
                (_, _) => false,
            };
            return result1 && result2;
        }
        (None, None) => {
            return true;
        }
        (_, _) => return false,
    };
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let buffer = vec![Some(1), Some(2), Some(3)];
        let p = build_tree(buffer);
        let buffer = vec![Some(1), Some(2), Some(3)];
        let q = build_tree(buffer);
        assert_eq!(is_same_tree(p, q), true);
    }
    #[test]
    fn run_tc2() {
        let buffer = vec![Some(1), Some(2)];
        let p = build_tree(buffer);
        let buffer = vec![Some(1), None, Some(3)];
        let q = build_tree(buffer);
        assert_eq!(is_same_tree(p, q), false);
    }

    #[test]
    fn run_tc3() {
        let buffer = vec![Some(1), Some(2), Some(1)];
        let p = build_tree(buffer);
        let buffer = vec![Some(1), Some(1), Some(2)];
        let q = build_tree(buffer);
        assert_eq!(is_same_tree(p, q), false);
    }
}

fn main() {
    let buffer = vec![Some(1), Some(2), Some(3)];
    let p = build_tree(buffer);
    let buffer = vec![Some(1), Some(2), Some(3)];
    let q = build_tree(buffer);
    assert_eq!(is_same_tree(p, q), true);
}
