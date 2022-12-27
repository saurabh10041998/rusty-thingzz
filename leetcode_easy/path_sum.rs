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
        let new_node: Option<Rc<RefCell<TreeNode>>>;
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

fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    if root.is_none() {
        return false;
    }
    has_path_sum_inner(root, target_sum)
}

fn has_path_sum_inner(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    if let Some(ref node) = root {
        let _node = node.borrow();
        if _node.val == target_sum && _node.left.is_none() && _node.right.is_none() {
            return true;
        }
        let result1 = match _node.left {
            Some(ref subnode) => {
                has_path_sum_inner(Some(Rc::clone(subnode)), target_sum - _node.val)
            }
            None => false,
        };
        let result2 = match _node.right {
            Some(ref subnode) => {
                has_path_sum_inner(Some(Rc::clone(subnode)), target_sum - _node.val)
            }
            None => false,
        };
        return result1 || result2;
    }
    false
}

#[cfg(test)]
pub mod tests {
    use crate::build_tree;
    use crate::has_path_sum;
    #[test]
    fn run_tc1() {
        let buffer = vec![
            Some(5),
            Some(4),
            Some(8),
            Some(11),
            None,
            Some(13),
            Some(4),
            Some(7),
            Some(2),
            None,
            None,
            None,
            Some(1),
        ];
        let root = build_tree(buffer);
        let target_sum = 22;
        assert_eq!(has_path_sum(root, target_sum), true);
    }
    #[test]
    fn run_tc2() {
        let buffer = vec![Some(1), Some(2), Some(3)];
        let root = build_tree(buffer);
        let target_sum = 5;
        assert_eq!(has_path_sum(root, target_sum), false);
    }
    #[test]
    fn run_tc3() {
        let buffer = vec![];
        let root = build_tree(buffer);
        let target_sum = 0;
        assert_eq!(has_path_sum(root, target_sum), false);
    }
}

fn main() {
    let buffer = vec![
        Some(5),
        Some(4),
        Some(8),
        Some(11),
        None,
        Some(13),
        Some(4),
        Some(7),
        Some(2),
        None,
        None,
        None,
        Some(1),
    ];
    let root = build_tree(buffer);
    let target_sum = 22;
    assert_eq!(has_path_sum(root, target_sum), true);
}
