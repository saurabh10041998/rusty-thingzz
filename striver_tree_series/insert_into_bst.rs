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

fn insert_into_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(ref root_node) = root {
        let mut curr = Some(Rc::clone(root_node));
        loop {
            if let Some(node) = curr {
                let mut _node = node.borrow_mut();
                if _node.val <= val {
                    match _node.right {
                        Some(ref subnode) => {
                            curr = Some(Rc::clone(subnode));
                        }
                        None => {
                            _node.right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                            break;
                        }
                    };
                } else {
                    match _node.left {
                        Some(ref subnode) => {
                            curr = Some(Rc::clone(subnode));
                        }
                        None => {
                            _node.left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                            break;
                        }
                    }
                }
            } else {
                break;
            }
        }
        return Some(Rc::clone(root_node));
    }
    Some(Rc::new(RefCell::new(TreeNode::new(val))))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let root = build_tree(vec![Some(4), Some(2), Some(7), Some(1), Some(3)]);
        let val = 5;
        assert_eq!(
            insert_into_bst(root, val),
            build_tree(vec![Some(4), Some(2), Some(7), Some(1), Some(3), Some(5)])
        );
    }
    #[test]
    fn run_tc2() {
        let root = build_tree(vec![
            Some(40),
            Some(20),
            Some(60),
            Some(10),
            Some(30),
            Some(50),
            Some(70),
        ]);
        let val = 25;
        assert_eq!(
            insert_into_bst(root, val),
            build_tree(vec![
                Some(40),
                Some(20),
                Some(60),
                Some(10),
                Some(30),
                Some(50),
                Some(70),
                None,
                None,
                Some(25)
            ])
        );
    }
    #[test]
    fn run_tc3() {
        let root = build_tree(vec![
            Some(4),
            Some(2),
            Some(7),
            Some(1),
            Some(3),
            None,
            None,
            None,
            None,
            None,
            None,
        ]);
        let val = 5;
        assert_eq!(
            insert_into_bst(root, val),
            build_tree(vec![Some(4), Some(2), Some(7), Some(1), Some(3), Some(5)])
        );
    }
}

fn main() {
    println!("Hello, world!");
}
