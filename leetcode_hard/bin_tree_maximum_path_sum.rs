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
    let root_rc = Rc::new(RefCell::new(TreeNode::new(buffer[0].unwrap())));
    let mut q = VecDeque::new();
    q.push_back(Rc::clone(&root_rc));
    let mut is_left = true;
    let mut curr = Rc::clone(&root_rc);
    for c in buffer.iter().skip(1) {
        let new_node;
        match c {
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
    Some(Rc::clone(&root_rc))
}

// Solution

fn gain_from_subtree(root: &Option<Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> i32 {
    if let Some(node) = root {
        let _node = node.borrow();
        let left_subtree_gain = match _node.left {
            Some(ref subnode) => i32::max(gain_from_subtree(&Some(Rc::clone(subnode)), max_sum), 0),
            None => 0,
        };
        let right_subtree_gain = match _node.right {
            Some(ref subnode) => i32::max(gain_from_subtree(&Some(Rc::clone(subnode)), max_sum), 0),
            None => 0,
        };

        *max_sum = i32::max(*max_sum, _node.val + left_subtree_gain + right_subtree_gain);

        return i32::max(
            _node.val + left_subtree_gain,
            _node.val + right_subtree_gain,
        );
    }
    0
}

fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max_sum = i32::MIN;
    gain_from_subtree(&root, &mut max_sum);
    max_sum
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let buffer = vec![Some(-10), Some(9), Some(20), None, None, Some(15), Some(7)];
        let root = build_tree(buffer);
        assert_eq!(max_path_sum(root), 42);
    }

    #[test]
    fn run_tc2() {
        let buffer = vec![Some(1), Some(2), Some(3)];
        let root = build_tree(buffer);
        assert_eq!(max_path_sum(root), 6);
    }
}

fn main() {
    let buffer = vec![Some(-10), Some(9), Some(20), None, None, Some(15), Some(7)];
    let root = build_tree(buffer);
    println!("Tree: {:#?}", root);
    assert_eq!(max_path_sum(root), 42);
}
