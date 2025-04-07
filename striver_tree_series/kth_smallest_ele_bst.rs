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

fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    if let Some(ref root_node) = root {
        let mut curr = Some(Rc::clone(root_node));
        let mut stack = vec![];
        let mut cnt = 0;

        loop {
            match curr {
                Some(node) => {
                    stack.push(Some(Rc::clone(&node)));
                    let _node = node.borrow();
                    match _node.left {
                        Some(ref subnode) => {
                            curr = Some(Rc::clone(subnode));
                        }
                        None => {
                            curr = None;
                        }
                    }
                }
                None => {
                    if stack.is_empty() {
                        break;
                    }
                    let maybe_next_node = stack.pop().unwrap();
                    if let Some(next_node) = maybe_next_node {
                        cnt += 1;
                        if cnt == k {
                            return next_node.borrow().val;
                        }
                        match next_node.borrow().right {
                            Some(ref subnode) => {
                                curr = Some(Rc::clone(subnode));
                            }
                            None => {
                                curr = None;
                            }
                        }
                    }
                }
            }
        }
        return -1;
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let root = build_tree(vec![Some(3), Some(1), Some(4), None, Some(2)]);
        let k = 1;
        assert_eq!(kth_smallest(root, k), 1);
    }
    #[test]
    fn run_tc2() {
        let root = build_tree(vec![
            Some(5),
            Some(3),
            Some(6),
            Some(2),
            Some(4),
            None,
            None,
            Some(1),
        ]);
        let k = 3;
        assert_eq!(kth_smallest(root, k), 3);
    }
}

fn main() {
    println!("Hello, world!");
}
