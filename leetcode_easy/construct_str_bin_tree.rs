use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
#[derive(Debug)]
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
    if buffer.len() == 0 {
        return None;
    }
    let root_rc = Rc::new(RefCell::new(TreeNode::new(buffer[0].unwrap())));
    let mut curr = Rc::clone(&root_rc);
    let mut q = VecDeque::new();
    q.push_back(Rc::clone(&root_rc));
    let mut left = true;
    for ele in buffer.into_iter().skip(1) {
        let new_node;
        match ele {
            Some(n) => {
                new_node = Some(Rc::new(RefCell::new(TreeNode::new(n))));
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

fn helper(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut String) {
    if let Some(node) = root {
        let _node = node.borrow();
        ans.push_str(format!("{}", _node.val).as_str());
        match _node.left {
            Some(ref subnode) => {
                ans.push_str("(");
                helper(&Some(Rc::clone(subnode)), ans);
                ans.push_str(")");
            }
            None => {}
        }
        match _node.right {
            Some(ref subnode) => {
                if _node.left.is_none() {
                    ans.push_str("()");
                }
                ans.push_str("(");
                helper(&Some(Rc::clone(subnode)), ans);
                ans.push_str(")");
            }
            None => {}
        }
    }
}

fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    let mut ans = String::new();
    helper(&root, &mut ans);
    ans
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let buffer = vec![Some(1), Some(2), Some(3), Some(4)];
        let root = build_tree(buffer);
        assert_eq!(tree2str(root).as_str(), "1(2(4))(3)");
    }
    #[test]
    fn run_tc2() {
        let buffer = vec![Some(1), Some(2), Some(3), None, Some(4)];
        let root = build_tree(buffer);
        assert_eq!(tree2str(root).as_str(), "1(2()(4))(3)");
    }
}

fn main() {
    let buffer = vec![Some(1), Some(2), Some(3), None, Some(4)];
    let root = build_tree(buffer);
    assert_eq!(tree2str(root).as_str(), "1(2()(4))(3)");
}
