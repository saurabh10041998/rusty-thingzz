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

fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    invert_tree_inner(root)
}

fn invert_tree_inner(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(ref node) = root {
        let mut _node = node.borrow_mut();
        let left_node = match _node.left {
            Some(ref subnode) => invert_tree_inner(Some(Rc::clone(subnode))),
            None => None,
        };
        let right_node = match _node.right {
            Some(ref subnode) => invert_tree_inner(Some(Rc::clone(subnode))),
            None => None,
        };
        _node.left = right_node;
        _node.right = left_node;
    }
    root
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let buffer = vec![
            Some(4),
            Some(2),
            Some(7),
            Some(1),
            Some(3),
            Some(6),
            Some(9),
        ];
        let root = build_tree(buffer);
        assert_eq!(
            invert_tree(root),
            build_tree(vec![
                Some(4),
                Some(7),
                Some(2),
                Some(9),
                Some(6),
                Some(3),
                Some(1)
            ])
        );
    }

    #[test]
    fn run_tc2() {
        let buffer = vec![Some(2), Some(1), Some(3)];
        let root = build_tree(buffer);
        assert_eq!(
            invert_tree(root),
            build_tree(vec![Some(2), Some(3), Some(1)])
        );
    }
}

fn main() {
    let buffer = vec![
        Some(4),
        Some(2),
        Some(7),
        Some(1),
        Some(3),
        Some(6),
        Some(9),
    ];
    let root = build_tree(buffer);
    assert_eq!(
        invert_tree(root),
        build_tree(vec![
            Some(4),
            Some(7),
            Some(2),
            Some(9),
            Some(6),
            Some(3),
            Some(1)
        ])
    );
}
