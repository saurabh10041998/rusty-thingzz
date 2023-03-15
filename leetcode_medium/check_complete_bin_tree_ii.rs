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

// ************* Solution **********************
fn count_nodes(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = root {
        let _node = node.borrow();
        let left_side = match _node.left {
            Some(ref subnode) => count_nodes(&Some(Rc::clone(subnode))),
            None => 0,
        };
        let right_side = match _node.right {
            Some(ref subnode) => count_nodes(&Some(Rc::clone(subnode))),
            None => 0,
        };
        return 1 + left_side + right_side;
    }
    0
}

fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let total = count_nodes(&root);
    dfs_helper(&root, 1, total)
}

fn dfs_helper(root: &Option<Rc<RefCell<TreeNode>>>, idx: i32, total: i32) -> bool {
    if let Some(node) = root {
        if idx > total {
            return false;
        }
        let _node = node.borrow();
        let result1 = match _node.left {
            Some(ref subnode) => dfs_helper(&Some(Rc::clone(subnode)), 2 * idx, total),
            None => true,
        };
        let result2 = match _node.right {
            Some(ref subnode) => dfs_helper(&Some(Rc::clone(subnode)), 2 * idx + 1, total),
            None => true,
        };
        return result1 && result2;
    }
    true
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let buffer = vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)];
        let root = build_tree(buffer);
        assert_eq!(is_complete_tree(root), true);
    }
    #[test]
    fn run_tc2() {
        let buffer = vec![Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(7)];
        let root = build_tree(buffer);
        assert_eq!(is_complete_tree(root), false);
    }
}

fn main() {
    let buffer = vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)];
    let root = build_tree(buffer);
    assert_eq!(is_complete_tree(root), true);
}
