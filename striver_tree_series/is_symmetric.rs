use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

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
    let mut is_left = true;
    q.push_back(Rc::clone(&root));
    let mut curr = Rc::clone(&root);

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
        if is_left {
            curr = q.pop_front().unwrap();
            curr.borrow_mut().left = new_node;
            is_left = false;
        } else {
            curr.borrow_mut().right = new_node;
            is_left = true;
        }
    }
    Some(root)
}

fn f(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (root1, root2) {
        (Some(root1), Some(root2)) => {
            let _root1_node = root1.borrow();
            let _root2_node = root2.borrow();

            let ans1 = match (_root1_node.left.as_ref(), _root2_node.right.as_ref()) {
                (Some(subnode1), Some(subnode2)) => {
                    f(Some(Rc::clone(subnode1)), Some(Rc::clone(subnode2)))
                }
                (None, None) => true,
                (_, _) => false,
            };
            let ans2 = match (_root1_node.right.as_ref(), _root2_node.left.as_ref()) {
                (Some(subnode1), Some(subnode2)) => {
                    f(Some(Rc::clone(subnode1)), Some(Rc::clone(subnode2)))
                }
                (None, None) => true,
                (_, _) => false,
            };
            return _root1_node.val == _root2_node.val && ans1 && ans2;
        }
        (None, None) => return true,
        (_, _) => return false,
    }
}

fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if let Some(ref root_node) = root {
        let _root_node = root_node.borrow();
        match (_root_node.left.as_ref(), _root_node.right.as_ref()) {
            (Some(left_node), Some(right_node)) => {
                return f(Some(Rc::clone(left_node)), Some(Rc::clone(right_node)))
            }
            (None, None) => return true,
            (_, _) => return false,
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let root = build_tree(vec![
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(4),
            Some(4),
            Some(3),
        ]);
        assert_eq!(is_symmetric(root), true);
    }

    #[test]
    fn run_tc2() {
        let root = build_tree(vec![
            Some(1),
            Some(2),
            Some(2),
            None,
            Some(3),
            None,
            Some(3),
        ]);
        assert_eq!(is_symmetric(root), false);
    }
}

fn main() {}
