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

fn solve(root: &Option<Rc<RefCell<TreeNode>>>, target: i32, res: &mut Vec<i32>) -> bool {
    if let Some(root_node) = root {
        let _root_node = root_node.borrow();
        res.push(_root_node.val);

        if _root_node.val == target {
            return true;
        }

        let ans1 = match _root_node.left {
            Some(ref subnode) => solve(&Some(Rc::clone(subnode)), target, res),
            None => false,
        };
        let ans2 = match _root_node.right {
            Some(ref subnode) => solve(&Some(Rc::clone(subnode)), target, res),
            None => false,
        };
        if ans1 || ans2 {
            return true;
        }
        res.pop().unwrap();
        return false;
    }
    false
}

fn path_from_root_node(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Vec<i32> {
    if let Some(ref root_node) = root {
        let mut res = vec![];
        solve(&Some(Rc::clone(root_node)), target, &mut res);
        return res;
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let root = build_tree(vec![Some(1), Some(2), Some(5), Some(3), Some(4)]);
        let target = 4;
        assert_eq!(path_from_root_node(root, target), vec![1, 2, 4]);
    }
}

fn main() {
    println!("Hello world");
}
