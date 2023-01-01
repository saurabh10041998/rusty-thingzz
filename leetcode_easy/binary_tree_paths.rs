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
            Some(val) => {
                new_node = Some(Rc::new(RefCell::new(TreeNode::new(*val))));
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

fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, paths: &mut Vec<Vec<i32>>, path: &mut Vec<i32>) {
    if let Some(node) = root {
        let _node = node.borrow();
        if _node.left.is_none() && _node.right.is_none() {
            path.push(_node.val);
            paths.push(path.clone());
            path.pop().unwrap();
            return;
        }
        path.push(_node.val);
        match _node.left {
            Some(ref subnode) => {
                dfs(&Some(Rc::clone(subnode)), paths, path);
            }
            None => {}
        };
        match _node.right {
            Some(ref subnode) => {
                dfs(&Some(Rc::clone(subnode)), paths, path);
            }
            None => {}
        };
        path.pop().unwrap();
    }
}

fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    let mut paths = vec![];
    let mut path = vec![];
    dfs(&root, &mut paths, &mut path);
    let mut ans = vec![];
    for p in paths {
        let mut path_str = String::new();
        for ele in p {
            path_str = path_str + "->" + &ele.to_string();
        }
        ans.push(String::from(&path_str[2..]));
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let buffer = vec![Some(1), Some(2), Some(3), None, Some(5)];
        let root = build_tree(buffer);
        assert_eq!(
            binary_tree_paths(root),
            vec![String::from("1->2->5"), String::from("1->3")]
        );
    }
    #[test]
    fn run_tc2() {
        let buffer = vec![Some(1)];
        let root = build_tree(buffer);
        assert_eq!(binary_tree_paths(root), vec![String::from("1")]);
    }
}
fn main() {
    let buffer = vec![Some(1), Some(2), Some(3), None, Some(5)];
    let root = build_tree(buffer);
    assert_eq!(
        binary_tree_paths(root),
        vec![String::from("1->2->5"), String::from("1->3")]
    );
}
