use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(PartialEq, Eq)]
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
    let mut curr = Rc::clone(&root_rc);
    let mut q = VecDeque::new();
    q.push_back(Rc::clone(&root_rc));
    let mut left = true;
    for ele in buffer.iter().skip(1) {
        let new_node;
        match ele {
            Some(x) => {
                new_node = Some(Rc::new(RefCell::new(TreeNode::new(*x))));
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

enum Direction {
    Left,
    Right,
}

fn helper(
    root: &Option<Rc<RefCell<TreeNode>>>,
    curr_length: i32,
    dir: Direction,
    max_length: &mut i32,
) {
    if let Some(node) = root {
        let _node = node.borrow();
        *max_length = i32::max(*max_length, curr_length);
        match _node.left {
            Some(ref subnode) => match dir {
                Direction::Left => {
                    helper(&Some(Rc::clone(subnode)), 1, Direction::Left, max_length);
                }
                Direction::Right => {
                    helper(
                        &Some(Rc::clone(subnode)),
                        curr_length + 1,
                        Direction::Left,
                        max_length,
                    );
                }
            },
            None => {}
        }
        match _node.right {
            Some(ref subnode) => match dir {
                Direction::Left => {
                    helper(
                        &Some(Rc::clone(subnode)),
                        curr_length + 1,
                        Direction::Right,
                        max_length,
                    );
                }
                Direction::Right => {
                    helper(&Some(Rc::clone(subnode)), 1, Direction::Right, max_length);
                }
            },
            None => {}
        }
    }
}

fn longest_zig_zag(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max_length = 0;
    helper(&root, 0, Direction::Left, &mut max_length);
    helper(&root, 0, Direction::Right, &mut max_length);
    max_length
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let buffer = vec![
            Some(1),
            None,
            Some(1),
            Some(1),
            Some(1),
            None,
            None,
            Some(1),
            Some(1),
            None,
            Some(1),
            None,
            None,
            None,
            Some(1),
            None,
            Some(1),
        ];
        let root = build_tree(buffer);
        assert_eq!(longest_zig_zag(root), 3);
    }
    #[test]
    fn run_tc2() {
        let buffer = vec![
            Some(1),
            Some(1),
            Some(1),
            None,
            Some(1),
            None,
            None,
            Some(1),
            Some(1),
            None,
            Some(1),
        ];
        let root = build_tree(buffer);
        assert_eq!(longest_zig_zag(root), 4);
    }
    #[test]
    fn run_tc3() {
        let buffer = vec![Some(1)];
        let root = build_tree(buffer);
        assert_eq!(longest_zig_zag(root), 0);
    }
}

fn main() {
    let buffer = vec![
        Some(1),
        None,
        Some(1),
        Some(1),
        Some(1),
        None,
        None,
        Some(1),
        Some(1),
        None,
        Some(1),
        None,
        None,
        None,
        Some(1),
        None,
        Some(1),
    ];
    let root = build_tree(buffer);
    assert_eq!(longest_zig_zag(root), 3);
}
