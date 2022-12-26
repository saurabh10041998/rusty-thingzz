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

fn is_symmetric_inner(
    left_node: &Option<Rc<RefCell<TreeNode>>>,
    right_node: &Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    match (left_node, right_node) {
        (Some(ref subnode1), Some(ref subnode2)) => {
            let _subnode1 = subnode1.borrow();
            let _subnode2 = subnode2.borrow();
            if _subnode1.val != _subnode2.val {
                return false;
            }
            let result1 = match (_subnode1.left.as_ref(), _subnode2.right.as_ref()) {
                (Some(e1), Some(e2)) => {
                    is_symmetric_inner(&Some(Rc::clone(e1)), &Some(Rc::clone(e2)))
                }
                (None, None) => true,
                (_, _) => false,
            };
            let result2 = match (_subnode1.right.as_ref(), _subnode2.left.as_ref()) {
                (Some(e1), Some(e2)) => {
                    is_symmetric_inner(&Some(Rc::clone(e1)), &Some(Rc::clone(e2)))
                }
                (None, None) => true,
                (_, _) => false,
            };
            return result1 && result2;
        }
        (None, None) => {
            return true;
        }
        (_, _) => {
            return false;
        }
    }
}

fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if let Some(ref node) = root {
        let _node = node.borrow();
        return is_symmetric_inner(&_node.left, &_node.right);
    }
    true
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let buffer = vec![
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(4),
            Some(4),
            Some(3),
        ];
        let root = build_tree(buffer);
        assert_eq!(is_symmetric(root), true);
    }

    #[test]
    fn run_tc2() {
        let buffer = vec![Some(1), Some(2), Some(2), None, Some(3), None, Some(3)];
        let root = build_tree(buffer);
        assert_eq!(is_symmetric(root), false);
    }
}

fn main() {
    let buffer = vec![
        Some(1),
        Some(2),
        Some(2),
        Some(3),
        Some(4),
        Some(4),
        Some(3),
    ];
    let root = build_tree(buffer);
    assert_eq!(is_symmetric(root), true);
}
