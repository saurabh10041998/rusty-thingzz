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

fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(root_node) = root {
        let mut q = VecDeque::new();
        let mut ans = 0;
        q.push_back((Rc::clone(&root_node), 0));

        while !q.is_empty() {
            let len = q.len();
            let mmin = q.front().unwrap().1;
            let mut first = i32::MAX;
            let mut last = i32::MIN;
            for idx in 0..len {
                let (node, val) = q.pop_front().unwrap();
                let cur_idx = val - mmin;
                if idx == 0 {
                    first = cur_idx;
                }
                if idx == len - 1 {
                    last = cur_idx;
                }
                let _node = node.borrow();
                match _node.left {
                    Some(ref subnode) => {
                        q.push_back((Rc::clone(subnode), 2 * cur_idx + 1));
                    }
                    None => {}
                };
                match _node.right {
                    Some(ref subnode) => {
                        q.push_back((Rc::clone(subnode), 2 * cur_idx + 2));
                    }
                    None => {}
                }
            }
            ans = i32::max(ans, last - first + 1);
        }
        return ans;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let root = build_tree(vec![
            Some(1),
            Some(3),
            Some(2),
            Some(5),
            Some(3),
            None,
            Some(9),
        ]);

        assert_eq!(width_of_binary_tree(root), 4);
    }
    #[test]
    fn run_tc2() {
        let root = build_tree(vec![
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
        ]);

        assert_eq!(width_of_binary_tree(root), 7);
    }
    #[test]
    fn run_tc3() {
        let root = build_tree(vec![Some(1), Some(3), Some(2), Some(5)]);

        assert_eq!(width_of_binary_tree(root), 2);
    }
}

fn main() {
    println!("Hello world");
}
