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
    let root = Rc::new(RefCell::new(TreeNode::new(buffer[0].unwrap())));
    let mut q = VecDeque::new();
    q.push_back(Rc::clone(&root));
    let mut curr = Rc::clone(&root);
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
    Some(root)
}

fn inorder_traversal_inner(root: Option<Rc<RefCell<TreeNode>>>, log: &mut Vec<i32>) {
    if let Some(ref node) = root {
        let n = node.borrow();
        match n.left {
            Some(ref subnode) => inorder_traversal_inner(Some(Rc::clone(subnode)), log),
            None => {}
        };
        log.push(n.val);
        match n.right {
            Some(ref subnode) => inorder_traversal_inner(Some(Rc::clone(subnode)), log),
            None => {}
        }
    }
}

fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut ans = vec![];
    inorder_traversal_inner(root, &mut ans);
    ans
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let buffer = vec![Some(1), None, Some(2), Some(3)];
        let root = build_tree(buffer);
        assert_eq!(inorder_traversal(root), vec![1, 3, 2]);
    }

    #[test]
    fn run_tc2() {
        let buffer = vec![];
        let root = build_tree(buffer);
        assert_eq!(inorder_traversal(root), vec![]);
    }

    #[test]
    fn run_tc3() {
        let buffer = vec![Some(1)];
        let root = build_tree(buffer);
        assert_eq!(inorder_traversal(root), vec![1]);
    }
}

fn main() {
    let buffer = vec![Some(1), None, Some(2), Some(3)];
    let root = build_tree(buffer);
    assert_eq!(inorder_traversal(root), vec![1, 3, 2]);
}
