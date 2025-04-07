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

fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(ref root_node) = root {
        let mut curr = Some(Rc::clone(root_node));
        while let Some(curr_node) = curr {
            let _curr_node = curr_node.borrow();
            if _curr_node.val == val {
                return Some(Rc::clone(&curr_node));
            }
            // move to right
            if _curr_node.val < val {
                match _curr_node.right {
                    Some(ref subnode) => curr = Some(Rc::clone(subnode)),
                    None => {
                        return None;
                    }
                };
            } else {
                // move to left
                match _curr_node.left {
                    Some(ref subnode) => {
                        curr = Some(Rc::clone(subnode));
                    }
                    None => return None,
                };
            }
        }
        return None;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let root = build_tree(vec![Some(4), Some(2), Some(7), Some(1), Some(3)]);
        assert_eq!(
            search_bst(root, 2),
            build_tree(vec![Some(2), Some(1), Some(3)])
        );
    }
    #[test]
    fn run_tc2() {
        let root = build_tree(vec![Some(4), Some(2), Some(7), Some(1), Some(3)]);
        assert_eq!(search_bst(root, 5), build_tree(vec![]));
    }
}

fn main() {
    println!("Hello, world!");
}
