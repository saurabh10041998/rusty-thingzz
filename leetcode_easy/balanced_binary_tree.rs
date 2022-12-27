use std::cell::RefCell;
use std::collections::VecDeque;
use std::ops::Sub;
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

fn abs<T: PartialOrd + Sub<Output = T>>(x: T, y: T) -> T {
    if x > y {
        return x - y;
    }
    y - x
}

fn is_balanced_inner(root: Option<Rc<RefCell<TreeNode>>>, result: &mut bool) -> i32 {
    if !*result || root.is_none() {
        return 0;
    }
    if let Some(ref node) = root {
        let _node = node.borrow();
        let result1 = match _node.left {
            Some(ref subnode) => is_balanced_inner(Some(Rc::clone(subnode)), result),
            None => 0,
        };
        let result2 = match _node.right {
            Some(ref subnode) => is_balanced_inner(Some(Rc::clone(subnode)), result),
            None => 0,
        };
        if abs(result1, result2) > 1 {
            *result = false;
            return 0;
        }
        return 1 + i32::max(result1, result2);
    }
    0
}

fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut balanced = true;
    is_balanced_inner(root, &mut balanced);
    balanced
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let buffer = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        let root = build_tree(buffer);
        assert_eq!(is_balanced(root), true);
    }

    #[test]
    fn run_tc2() {
        let buffer = vec![
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(3),
            None,
            None,
            Some(4),
            Some(4),
        ];
        let root = build_tree(buffer);
        assert_eq!(is_balanced(root), false);
    }
}

fn main() {
    let buffer = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
    let root = build_tree(buffer);
    assert_eq!(is_balanced(root), true);
}
