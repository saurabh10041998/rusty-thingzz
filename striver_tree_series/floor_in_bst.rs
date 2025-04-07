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

fn find_floor(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> i32 {
    if let Some(ref root_node) = root {
        let mut curr = Some(Rc::clone(root_node));
        let mut floor = -1;
        while let Some(node) = curr {
            let _node = node.borrow();
            if _node.val == key {
                floor = _node.val;
                return floor;
            } else if _node.val > key {
                // move left
                match _node.left {
                    Some(ref subnode) => {
                        curr = Some(Rc::clone(subnode));
                    }
                    None => return floor,
                };
            } else {
                floor = _node.val;
                match _node.right {
                    Some(ref subnode) => {
                        curr = Some(Rc::clone(subnode));
                    }
                    None => return floor,
                };
            }
        }
        return floor;
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let root = build_tree(vec![Some(10), Some(5), Some(15), Some(2), Some(6)]);
        assert_eq!(find_floor(root, 7), 6);
    }
    #[test]
    fn run_tc2() {
        let root = build_tree(vec![Some(2), Some(1), Some(3)]);
        assert_eq!(find_floor(root, 2), 2);
    }
}

fn main() {
    println!("Hello, world!");
}
