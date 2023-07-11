use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
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

// To Construct the BinTree from Array of int
fn build_tree(buffer: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if buffer.is_empty() {
        return None;
    }
    let root_rc = Rc::new(RefCell::new(TreeNode::new(buffer[0].unwrap())));
    let mut q = VecDeque::new();
    q.push_back(Rc::clone(&root_rc));
    let mut left = true;
    let mut curr = Rc::clone(&root_rc);
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
        match left {
            true => {
                curr = q.pop_front().unwrap();
                curr.borrow_mut().left = new_node;
                left = false;
            }
            false => {
                curr.borrow_mut().right = new_node;
                left = true;
            }
        }
    }
    Some(root_rc)
}

// Solution
fn distance_k(
    root: Option<Rc<RefCell<TreeNode>>>,
    target: Option<Rc<RefCell<TreeNode>>>,
    k: i32,
) -> Vec<i32> {
    if let Some(ref _root) = root {
        let mut q = VecDeque::new();
        q.push_back(Rc::clone(_root));
        let mut parent = HashMap::new();
        while !q.is_empty() {
            let sz = q.len();
            for _ in 0..sz {
                let curr = q.pop_front().unwrap();
                let _node = curr.borrow();
                if let Some(ref subnode) = _node.left {
                    parent
                        .entry(subnode.borrow().val)
                        .or_insert(Rc::clone(&curr));
                    q.push_back(Rc::clone(subnode));
                }
                if let Some(ref subnode) = _node.right {
                    parent
                        .entry(subnode.borrow().val)
                        .or_insert(Rc::clone(&curr));
                    q.push_back(Rc::clone(subnode));
                }
            }
        }
        let mut q = VecDeque::new();
        let mut visited = HashSet::new();
        q.push_back(Rc::clone(target.as_ref().unwrap()));
        let mut k = k;
        while k > 0 && !q.is_empty() {
            let sz = q.len();
            for _ in 0..sz {
                let curr = q.pop_front().unwrap();
                let _node = curr.borrow();
                visited.insert(_node.val);
                if let Some(ref subnode) = _node.left {
                    if !visited.contains(&subnode.borrow().val) {
                        q.push_back(Rc::clone(subnode));
                    }
                }
                if let Some(ref subnode) = _node.right {
                    if !visited.contains(&subnode.borrow().val) {
                        q.push_back(Rc::clone(subnode));
                    }
                }
                if let Some(parent_node) = parent.get(&_node.val) {
                    if !visited.contains(&parent_node.borrow().val) {
                        q.push_back(Rc::clone(parent_node));
                    }
                }
            }
            k -= 1;
        }
        println!("{:?}", q);
        let mut ans = vec![];
        while let Some(node) = q.pop_front() {
            ans.push(node.borrow().val);
        }
        return ans;
    }
    vec![]
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let buffer = vec![
            Some(3),
            Some(5),
            Some(1),
            Some(6),
            Some(2),
            Some(0),
            Some(8),
            None,
            None,
            Some(7),
            Some(4),
        ];
        let root = build_tree(buffer);
        let target = build_tree(vec![
            Some(5),
            Some(6),
            Some(2),
            None,
            None,
            Some(7),
            Some(4),
        ]);
        let k = 2;
        assert_eq!(distance_k(root, target, k), vec![7, 4, 1]);
    }
}

fn main() {
    let buffer = vec![
        Some(3),
        Some(5),
        Some(1),
        Some(6),
        Some(2),
        Some(0),
        Some(8),
        None,
        None,
        Some(7),
        Some(4),
    ];
    let root = build_tree(buffer);
    let target = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    let k = 2;
    assert_eq!(distance_k(root, target, k), vec![7, 4, 1]);
}
