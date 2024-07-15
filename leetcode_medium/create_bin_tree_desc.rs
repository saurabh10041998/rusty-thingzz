use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};
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

// For testing purpose
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
        let new_node: Option<Rc<RefCell<TreeNode>>>;
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

//******************************* */
// Answer
//******************************* */
fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut hs: HashMap<i32, Rc<RefCell<TreeNode>>> = HashMap::new();
    let mut set: HashSet<i32> = HashSet::new();
    for d in descriptions.iter() {
        let parent = d[0];
        let child = d[1];
        let is_left = d[2];
        let parent_node = match hs.get(&parent) {
            Some(node) => Rc::clone(node),
            None => Rc::new(RefCell::new(TreeNode::new(parent))),
        };
        let child_node = match hs.get(&child) {
            Some(node) => Rc::clone(node),
            None => Rc::new(RefCell::new(TreeNode::new(child))),
        };
        set.insert(child);
        if is_left == 1 {
            parent_node.borrow_mut().left = Some(child_node);
            hs.entry(child)
                .and_modify(|node| *node = Rc::clone(parent_node.borrow().left.as_ref().unwrap()))
                .or_insert(Rc::clone(parent_node.borrow().left.as_ref().unwrap()));
        } else {
            parent_node.borrow_mut().right = Some(child_node);
            hs.entry(child)
                .and_modify(|node| *node = Rc::clone(parent_node.borrow().right.as_ref().unwrap()))
                .or_insert(Rc::clone(parent_node.borrow().right.as_ref().unwrap()));
        }
        hs.entry(parent)
            .and_modify(|node| *node = Rc::clone(&parent_node))
            .or_insert(parent_node);
    }
    for d in descriptions {
        if !set.contains(&d[0]) {
            return Some(Rc::clone(hs.get(&d[0]).unwrap()));
        }
    }
    None
}

#[cfg(test)]
pub mod tests {
    use crate::{build_tree, create_binary_tree};

    #[test]
    fn run_tc1() {
        let descriptions = vec![
            vec![20, 15, 1],
            vec![20, 17, 0],
            vec![50, 20, 1],
            vec![50, 80, 0],
            vec![80, 19, 1],
        ];
        let buffer = vec![Some(50), Some(20), Some(80), Some(15), Some(17), Some(19)];
        assert_eq!(create_binary_tree(descriptions), build_tree(buffer));
    }
    #[test]
    fn run_tc2() {
        let descriptions = vec![vec![1, 2, 1], vec![2, 3, 0], vec![3, 4, 1]];
        let buffer = vec![Some(1), Some(2), None, None, Some(3), Some(4)];
        assert_eq!(create_binary_tree(descriptions), build_tree(buffer));
    }
}

fn main() {
    let descriptions = vec![
        vec![20, 15, 1],
        vec![20, 17, 0],
        vec![50, 20, 1],
        vec![50, 80, 0],
        vec![80, 19, 1],
    ];
    let buffer = vec![Some(50), Some(20), Some(80), Some(15), Some(17), Some(19)];
    assert_eq!(create_binary_tree(descriptions), build_tree(buffer))
}
