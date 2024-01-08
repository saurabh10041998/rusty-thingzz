use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl From<i32> for TreeNode {
    fn from(val: i32) -> Self {
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
    let root_rc = Rc::new(RefCell::new(TreeNode::from(buffer[0].unwrap())));
    let mut left = true;
    let mut q = VecDeque::new();
    let mut curr = Rc::clone(&root_rc);
    q.push_back(Rc::clone(&root_rc));
    for ele in buffer.into_iter().skip(1) {
        let new_node: Option<Rc<RefCell<TreeNode>>>;
        match ele {
            Some(val) => {
                new_node = Some(Rc::new(RefCell::new(val.into())));
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

fn range_sum_bst_helper(root: Option<&Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    if let Some(node) = root {
        let _node = node.borrow();
        let mut sum = 0;
        if _node.val >= low && _node.val <= high {
            sum += _node.val;
        }
        sum += match _node.left {
            Some(ref subnode) => range_sum_bst_helper(Some(&Rc::clone(subnode)), low, high),
            None => 0,
        };
        sum += match _node.right {
            Some(ref subnode) => range_sum_bst_helper(Some(&Rc::clone(subnode)), low, high),
            None => 0,
        };
        return sum;
    }
    0
}

fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    range_sum_bst_helper(root.as_ref(), low, high)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let buffer = vec![
            Some(10),
            Some(5),
            Some(15),
            Some(3),
            Some(7),
            None,
            Some(18),
        ];
        let root = build_tree(buffer);
        assert_eq!(range_sum_bst(root, 7, 15), 32);
    }
    #[test]
    fn run_tc2() {
        let buffer = vec![
            Some(10),
            Some(5),
            Some(15),
            Some(3),
            Some(7),
            Some(13),
            Some(18),
            Some(1),
            None,
            Some(6),
        ];
        let root = build_tree(buffer);
        assert_eq!(range_sum_bst(root, 6, 10), 23);
    }
}

fn main() {
    let buffer = vec![
        Some(10),
        Some(5),
        Some(15),
        Some(3),
        Some(7),
        Some(13),
        Some(18),
        Some(1),
        None,
        Some(6),
    ];
    let root = build_tree(buffer);
    assert_eq!(range_sum_bst(root, 6, 10), 23);
}
