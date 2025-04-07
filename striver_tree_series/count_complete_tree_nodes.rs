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

fn find_height_left(root: Rc<RefCell<TreeNode>>) -> i32 {
    let mut lh = 1;
    let mut curr = Rc::clone(&root);
    while curr.borrow().left.is_some() {
        lh += 1;
        let new_node = Rc::clone(curr.borrow().left.as_ref().unwrap());
        curr = new_node;
    }
    lh
}

fn find_height_right(root: Rc<RefCell<TreeNode>>) -> i32 {
    let mut rh = 0;
    let mut curr = Rc::clone(&root);
    while curr.borrow().right.is_some() {
        rh += 1;
        let new_node = Rc::clone(curr.borrow().right.as_ref().unwrap());
        curr = new_node;
    }
    rh
}

fn count_node_inner(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(root_node) = root {
        let left_height = find_height_left(Rc::clone(root_node));
        let right_height = find_height_right(Rc::clone(root_node));

        if left_height == right_height {
            return (1 << left_height) - 1;
        }

        let _root_node = root_node.borrow();
        let left = match _root_node.left {
            Some(ref subnode) => count_node_inner(&Some(Rc::clone(subnode))),
            None => 0,
        };
        let right = match _root_node.right {
            Some(ref subnode) => count_node_inner(&Some(Rc::clone(subnode))),
            None => 0,
        };
        return 1 + left + right;
    }
    0
}

fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(ref root_node) = root {
        return count_node_inner(&Some(Rc::clone(root_node)));
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let root = build_tree(vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)]);
        assert_eq!(count_nodes(root), 6);
    }
}

fn main() {
    println!("Hello, world!");
}
