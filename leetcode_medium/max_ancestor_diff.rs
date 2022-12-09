use std::cell::RefCell;
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

fn insert(mut root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if root.is_none() {
        let new_node = TreeNode::new(val);
        root = Some(Rc::new(RefCell::new(new_node)));
        return root;
    }
    if let Some(ref node) = root {
        let mut _node = node.borrow_mut();
        if _node.val > val {
            match _node.left.as_ref() {
                Some(n) => {
                    _node.left = insert(Some(Rc::clone(n)), val);
                }
                None => {
                    _node.left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                }
            }
        } else {
            match _node.right.as_ref() {
                Some(n) => {
                    _node.right = insert(Some(Rc::clone(n)), val);
                }
                None => {
                    _node.right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                }
            }
        }
    }
    root
}

fn traverse(root: &Option<Rc<RefCell<TreeNode>>>) {
    if let Some(node) = root {
        let _node = node.borrow();
        match _node.left.as_ref() {
            Some(n) => {
                traverse(&Some(Rc::clone(n)));
            }
            None => {}
        }
        println!("{:?}", _node.val);
        match _node.right.as_ref() {
            Some(n) => {
                traverse(&Some(Rc::clone(n)));
            }
            None => {}
        }
    }
}

fn abs<T: PartialOrd + Sub<Output = T>>(x: T, y: T) -> T {
    if x > y {
        return x - y;
    }
    y - x
}

fn preorder(root: Option<Rc<RefCell<TreeNode>>>, max_diff: &mut i32) -> (i32, i32) {
    if let Some(ref node) = root {
        let _node = node.borrow();
        if _node.left.is_none() && _node.right.is_none() {
            return (_node.val, _node.val);
        }
        let mut left = (0, 0);
        let mut right = (0, 0);
        match _node.left.as_ref() {
            Some(n) => {
                left = preorder(Some(Rc::clone(n)), max_diff);
            }
            None => {
                // dont take anything from None tree
                left = (i32::MAX, i32::MIN);
            }
        }
        match _node.right.as_ref() {
            Some(n) => {
                right = preorder(Some(Rc::clone(n)), max_diff);
            }
            None => {
                // dont take anything from None tree
                right = (i32::MAX, i32::MIN);
            }
        }

        let mut maxx = i32::max(right.1, left.1);
        let mut minn = i32::min(right.0, left.0);
        *max_diff = i32::max(
            *max_diff,
            i32::max(abs(maxx, _node.val), abs(_node.val, minn)),
        );

        maxx = i32::max(_node.val, maxx);
        minn = i32::min(_node.val, minn);
        return (minn, maxx);
    }
    (i32::MAX, i32::MIN)
}

// Solution
fn max_ancestor_diff(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max_diff = 0;
    preorder(root, &mut max_diff);
    max_diff
}

fn create_tree(buffer: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut root = None;
    for i in buffer {
        root = insert(root, i);
    }
    root
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let nums = vec![8, 3, 10, 1, 6, 14, 4, 7, 13];
        let root = create_tree(nums);
        assert_eq!(max_ancestor_diff(root), 7);
    }

    #[test]
    fn run_tc2() {
        let nums = vec![0, 1, 2, 3];
        let root = create_tree(nums);
        assert_eq!(max_ancestor_diff(root), 3);
    }
}

fn main() {
    let nums = vec![8, 3, 10, 1, 6, 14, 4, 7, 13];
    let root = create_tree(nums);
    traverse(&root);
    assert_eq!(max_ancestor_diff(root), 7);
}
