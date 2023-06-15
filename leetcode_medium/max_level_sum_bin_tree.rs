use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
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
    let root_rc = Rc::new(RefCell::new(TreeNode::new(buffer[0].unwrap())));
    let mut q = VecDeque::new();
    let mut curr = Rc::clone(&root_rc);
    let mut left = true;
    q.push_back(Rc::clone(&root_rc));
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
    Some(root_rc)
}

// ************** Solution *****************
fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(root_rc) = root {
        let mut q = VecDeque::new();
        q.push_back(Rc::clone(&root_rc));
        let mut sum = i64::MIN;
        let mut level = 0;
        let mut ans = 0;
        while !q.is_empty() {
            let len = q.len();
            level += 1;
            let mut temp = 0;
            for _ in 0..len {
                let curr = q.pop_front().unwrap();
                let _curr = curr.borrow();
                temp += _curr.val as i64;
                if let Some(ref subnode) = _curr.left {
                    q.push_back(Rc::clone(subnode))
                }
                if let Some(ref subnode) = _curr.right {
                    q.push_back(Rc::clone(subnode))
                }
            }
            if temp > sum {
                sum = temp;
                ans = level;
            }
        }
        return ans;
    }
    unreachable!();
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let buffer = vec![Some(1), Some(7), Some(0), Some(7), Some(-8), None, None];
        assert_eq!(max_level_sum(build_tree(buffer)), 2);
    }
    #[test]
    fn run_tc2() {
        let buffer = vec![
            Some(989),
            None,
            Some(10250),
            Some(98693),
            Some(-89388),
            None,
            None,
            Some(-32127),
        ];
        assert_eq!(max_level_sum(build_tree(buffer)), 2);
    }
}

fn main() {
    let buffer = vec![
        Some(989),
        None,
        Some(10250),
        Some(98693),
        Some(-89388),
        None,
        None,
        Some(-32127),
    ];
    assert_eq!(max_level_sum(build_tree(buffer)), 2);
}
