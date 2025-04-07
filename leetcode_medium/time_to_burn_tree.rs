use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
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

#[derive(Debug, Eq, PartialEq)]
struct NodeKey(Rc<RefCell<TreeNode>>);

impl Hash for NodeKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // hashing based on the memory address of the pointer
        std::ptr::hash(&*self.0, state);
    }
}

fn build_tree(buffer: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if buffer.is_empty() {
        return None;
    }
    let root = Rc::new(RefCell::new(TreeNode::new(buffer[0].unwrap())));
    let mut q = VecDeque::new();
    let mut curr = Rc::clone(&root);
    q.push_back(Rc::clone(&root));
    let mut left = true;
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
    Some(root)
}

fn find_node(root: &Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(root_node) = root {
        let _root_node = root_node.borrow();
        if _root_node.val == target {
            return Some(Rc::clone(root_node));
        }
        let ans1 = match _root_node.left {
            Some(ref subnode) => find_node(&Some(Rc::clone(subnode)), target),
            None => None,
        };
        if ans1.is_some() {
            return ans1;
        }
        let ans2 = match _root_node.right {
            Some(ref subnode) => find_node(&Some(Rc::clone(subnode)), target),
            None => None,
        };
        if ans2.is_some() {
            return ans2;
        }
        return None;
    }
    None
}

fn map_to_parents(
    root: &Option<Rc<RefCell<TreeNode>>>,
    mpp: &mut HashMap<NodeKey, Rc<RefCell<TreeNode>>>,
) {
    if let Some(root_node) = root {
        let mut q = VecDeque::new();
        q.push_back(Rc::clone(root_node));
        while let Some(node) = q.pop_front() {
            let _node = node.borrow();
            match _node.left {
                Some(ref subnode) => {
                    mpp.entry(NodeKey(Rc::clone(subnode)))
                        .or_insert(Rc::clone(&node));
                    q.push_back(Rc::clone(subnode));
                }
                None => {}
            }
            match _node.right {
                Some(ref subnode) => {
                    mpp.entry(NodeKey(Rc::clone(subnode)))
                        .or_insert(Rc::clone(&node));
                    q.push_back(Rc::clone(subnode));
                }
                None => {}
            };
        }
    }
}

fn time_to_burn_tree(
    root: Option<Rc<RefCell<TreeNode>>>,
    target: Option<Rc<RefCell<TreeNode>>>,
) -> i32 {
    if let (Some(ref root_node), Some(ref target_node)) = (root, target) {
        let mut mpp: HashMap<NodeKey, Rc<RefCell<TreeNode>>> = HashMap::new();
        map_to_parents(&Some(Rc::clone(root_node)), &mut mpp);
        let mut q = VecDeque::new();
        q.push_back(Rc::clone(target_node));
        let mut visited: HashMap<NodeKey, bool> = HashMap::new();
        let mut time = 0;
        while !q.is_empty() {
            let len = q.len();
            let mut remaining = false;
            for _ in 0..len {
                let node = q.pop_front().unwrap();
                visited.entry(NodeKey(Rc::clone(&node))).or_insert(true);
                let _node = node.borrow();
                match _node.left {
                    Some(ref subnode) => {
                        if !visited.contains_key(&NodeKey(Rc::clone(subnode))) {
                            q.push_back(Rc::clone(subnode));
                            remaining = true;
                        }
                    }
                    None => {}
                }
                match _node.right {
                    Some(ref subnode) => {
                        if !visited.contains_key(&NodeKey(Rc::clone(subnode))) {
                            q.push_back(Rc::clone(subnode));
                            remaining = true;
                        }
                    }
                    None => {}
                }
                match mpp.get(&NodeKey(Rc::clone(&node))) {
                    Some(parent) => {
                        if !visited.contains_key(&NodeKey(Rc::clone(parent))) {
                            q.push_back(Rc::clone(parent));
                            remaining = true;
                        }
                    }
                    None => {}
                }
            }
            if remaining {
                time += 1;
            }
        }
        return time;
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
            Some(2),
            Some(3),
            None,
            None,
            Some(4),
            Some(5),
        ]);
        let target = find_node(&root, 3);
        assert_eq!(time_to_burn_tree(root, target), 2);
    }
}

fn main() {
    println!("Hello, world!");
}
