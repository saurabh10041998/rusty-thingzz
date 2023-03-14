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

// ********************** Solution **************************
fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(root_node) => {
            let mut stack = Vec::new();
            stack.push((Rc::clone(&root_node), 0));
            let mut res = 0;
            while let Some(node) = stack.pop() {
                let _node = node.0.borrow();
                let curr_sum = 10 * node.1 + _node.val;
                // Leaf node
                if _node.left.is_none() && _node.right.is_none() {
                    res += curr_sum;
                    continue;
                }
                match _node.left {
                    Some(ref subnode) => stack.push((Rc::clone(subnode), curr_sum)),
                    None => {}
                };
                match _node.right {
                    Some(ref subnode) => stack.push((Rc::clone(subnode), curr_sum)),
                    None => {}
                };
            }
            res
        }
        None => 0,
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let buffer = vec![Some(1), Some(2), Some(3)];
        let root = build_tree(buffer);
        assert_eq!(sum_numbers(root), 25);
    }
    #[test]
    fn run_tc2() {
        let buffer = vec![Some(4), Some(9), Some(0), Some(5), Some(1)];
        let root = build_tree(buffer);
        assert_eq!(sum_numbers(root), 1026);
    }
}

fn main() {
    let buffer = vec![Some(4), Some(9), Some(0), Some(5), Some(1)];
    let root = build_tree(buffer);
    assert_eq!(sum_numbers(root), 1026);
}
