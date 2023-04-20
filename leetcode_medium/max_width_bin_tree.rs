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
    let mut left = true;
    q.push_back(Rc::clone(&root_rc));
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

fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node) = root {
        let mut q = VecDeque::new();
        let mut ans = 0;
        q.push_back((Rc::clone(&node), 0));
        while !q.is_empty() {
            let mut childs = VecDeque::new();
            let min = q.front().unwrap().1;
            let mut first = i32::MAX;
            let mut last = i32::MIN;
            while let Some(pair) = q.pop_front() {
                let _node = pair.0.borrow();
                let curr_id = pair.1;
                first = i32::min(curr_id, first);
                last = i32::max(curr_id, last);
                let new_id = curr_id - min;
                match _node.left {
                    Some(ref subnode) => {
                        childs.push_back((Rc::clone(subnode), 2 * new_id + 1));
                    }
                    None => {}
                };
                match _node.right {
                    Some(ref subnode) => {
                        childs.push_back((Rc::clone(subnode), 2 * new_id + 2));
                    }
                    None => {}
                }
            }
            ans = i32::max(ans, last - first + 1);
            q = std::mem::take(&mut childs);
        }
        return ans;
    }
    0
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let buffer = vec![Some(1), Some(3), Some(2), Some(5), Some(3), None, Some(9)];
        let root = build_tree(buffer);
        assert_eq!(width_of_binary_tree(root), 4);
    }
    #[test]
    fn run_tc2() {
        let buffer = vec![
            Some(1),
            Some(3),
            Some(2),
            Some(5),
            None,
            None,
            Some(9),
            Some(6),
            None,
            Some(7),
        ];
        let root = build_tree(buffer);
        assert_eq!(width_of_binary_tree(root), 7);
    }
    #[test]
    fn run_tc3() {
        let buffer = vec![Some(1), Some(3), Some(2), Some(5)];
        let root = build_tree(buffer);
        assert_eq!(width_of_binary_tree(root), 2);
    }
}

fn main() {
    let buffer = vec![Some(1), Some(3), Some(2), Some(5), Some(3), None, Some(9)];
    let root = build_tree(buffer);
    assert_eq!(width_of_binary_tree(root), 4);
}
