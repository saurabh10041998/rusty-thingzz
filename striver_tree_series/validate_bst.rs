use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, Eq, PartialEq)]
struct TreeNode {
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
    let root = Rc::new(RefCell::new(TreeNode::new(buffer[0].unwrap())));
    let mut q = VecDeque::new();
    let mut curr = Rc::clone(&root);
    q.push_back(Rc::clone(&root));
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
    Some(root)
}

fn is_valid_bst_inner(root: &Option<Rc<RefCell<TreeNode>>>, minval: i64, maxval: i64) -> bool {
    if let Some(root_node) = root {
        let _root_node = root_node.borrow();
        if _root_node.val as i64 >= maxval || _root_node.val as i64 <= minval {
            return false;
        }
        let ans1 = match _root_node.left {
            Some(ref subnode) => {
                is_valid_bst_inner(&Some(Rc::clone(subnode)), minval, _root_node.val as i64)
            }
            None => true,
        };
        let ans2 = match _root_node.right {
            Some(ref subnode) => {
                is_valid_bst_inner(&Some(Rc::clone(subnode)), _root_node.val as i64, maxval)
            }
            None => true,
        };
        return ans1 && ans2;
    }
    true
}

fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if let Some(ref root_node) = root {
        return is_valid_bst_inner(&Some(Rc::clone(root_node)), i64::MIN, i64::MAX);
    }
    true
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn run_tc1() {
        let root = build_tree(vec![Some(2), Some(1), Some(3)]);
        assert_eq!(is_valid_bst(root), true);
    }
    #[test]
    fn run_tc2() {
        let root = build_tree(vec![
            Some(5),
            Some(1),
            Some(4),
            None,
            None,
            Some(3),
            Some(6),
        ]);
        assert_eq!(is_valid_bst(root), false);
    }
}

fn main() {
    println!("Hello, world!");
}
