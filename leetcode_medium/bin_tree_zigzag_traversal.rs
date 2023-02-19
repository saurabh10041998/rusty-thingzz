use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug)]
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
    let mut q = VecDeque::new();
    q.push_back(Rc::clone(&root_rc));
    let mut left = true;
    let mut curr = Rc::clone(&root_rc);
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

// ************ Solution ***************
fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    if let Some(ref node) = root {
        let mut q = VecDeque::new();
        q.push_back(Rc::clone(node));
        let mut level = 1;
        while !q.is_empty() {
            let len = q.len();
            let mut ele = vec![];
            for _ in 0..len {
                let curr = q.pop_front().unwrap();
                let _node = curr.borrow();
                ele.push(_node.val);
                match _node.left {
                    Some(ref subnode) => q.push_back(Rc::clone(subnode)),
                    None => {}
                };
                match _node.right {
                    Some(ref subnode) => q.push_back(Rc::clone(subnode)),
                    None => {}
                };
            }
            let r = match level % 2 == 0 {
                true => ele.iter().rev().map(|x| *x).collect::<Vec<i32>>(),
                false => ele,
            };
            res.push(r);
            level += 1;
        }
    }
    res
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let buffer = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
        let root = build_tree(buffer);
        assert_eq!(
            zigzag_level_order(root),
            vec![vec![3], vec![20, 9], vec![15, 7]]
        );
    }
    #[test]
    fn run_tc2() {
        let buffer = vec![Some(1)];
        let root = build_tree(buffer);
        assert_eq!(zigzag_level_order(root), vec![vec![1]]);
    }
}

fn main() {
    let buffer = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
    let root = build_tree(buffer);
    assert_eq!(
        zigzag_level_order(root),
        vec![vec![3], vec![20, 9], vec![15, 7]]
    );
}
