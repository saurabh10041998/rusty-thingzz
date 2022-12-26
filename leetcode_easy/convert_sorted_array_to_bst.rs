use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
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
    let root_node = Rc::new(RefCell::new(TreeNode::new(buffer[0].unwrap())));
    let mut curr = Rc::clone(&root_node);
    let mut is_left = true;
    let mut q = VecDeque::new();
    q.push_back(Rc::clone(&root_node));
    for ele in buffer.iter().skip(1) {
        let new_node;
        match ele {
            Some(v) => {
                new_node = Some(Rc::new(RefCell::new(TreeNode::new(*v))));
                q.push_back(Rc::clone(new_node.as_ref().unwrap()));
            }
            None => {
                new_node = None;
            }
        };
        if is_left {
            curr = q.pop_front().unwrap();
            curr.borrow_mut().left = new_node;
            is_left = false;
        } else {
            curr.borrow_mut().right = new_node;
            is_left = true;
        }
    }
    Some(root_node)
}

fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    build_bst(&nums, 0, nums.len() - 1)
}

fn build_bst(nums: &Vec<i32>, left: usize, right: usize) -> Option<Rc<RefCell<TreeNode>>> {
    if left > right {
        return None;
    }
    let mid = left + (right - left) / 2;
    let root_node = Rc::new(RefCell::new(TreeNode::new(nums[mid])));
    root_node.borrow_mut().left = match mid.checked_sub(1) {
        Some(new_mid) => build_bst(nums, left, new_mid),
        None => None
    };
    root_node.borrow_mut().right = build_bst(nums, mid + 1, right);
    Some(root_node)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let nums = vec![-10,-3,0,5,9];
        // likely to be fail
        assert_eq!(sorted_array_to_bst(nums),  build_tree(vec![Some(0), Some(-3), Some(5), Some(-10), None, Some(9)]));
    }
    #[test]
    fn run_tc2() {
        let nums = vec![1, 3];
        // likely to be fail
        assert_eq!(sorted_array_to_bst(nums),  build_tree(vec![Some(1), Some(3)]));
    }
}
fn main() {

}