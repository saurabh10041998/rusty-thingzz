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

fn find_last_right(root: Rc<RefCell<TreeNode>>) -> Rc<RefCell<TreeNode>> {
    let _root_node = root.borrow();
    match _root_node.right {
        Some(ref subnode) => {
            return find_last_right(Rc::clone(subnode));
        }
        None => {
            return Rc::clone(&root);
        }
    }
}
fn helper(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(root_node) = root {
        let _root_node = root_node.borrow();
        if _root_node.left.is_none() {
            match _root_node.right {
                Some(ref subnode) => {
                    return Some(Rc::clone(subnode));
                }
                None => return None,
            }
        }

        if _root_node.right.is_none() {
            match _root_node.left {
                Some(ref subnode) => {
                    return Some(Rc::clone(subnode));
                }
                None => return None,
            }
        }

        // both are is_some
        let right_child = Rc::clone(_root_node.right.as_ref().unwrap());
        let last_right = find_last_right(Rc::clone(_root_node.left.as_ref().unwrap()));
        let mut _last_right = last_right.borrow_mut();
        _last_right.right = Some(right_child);
        return Some(Rc::clone(_root_node.left.as_ref().unwrap()));
    }
    None
}

fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(ref root_node) = root {
        let mut curr = Some(Rc::clone(root_node));
        while let Some(node) = curr {
            if node.borrow().val == key {
                return helper(&Some(Rc::clone(&node)));
            }
            let mut _node = node.borrow_mut();
            if _node.val > key {
                match _node.left {
                    Some(ref subnode) => {
                        if subnode.borrow().val == key {
                            _node.left = helper(&Some(Rc::clone(subnode)));
                            break;
                        } else {
                            curr = Some(Rc::clone(subnode));
                        }
                    }
                    None => break,
                }
            } else {
                match _node.right {
                    Some(ref subnode) => {
                        if subnode.borrow().val == key {
                            _node.right = helper(&Some(Rc::clone(subnode)));
                            break;
                        } else {
                            curr = Some(Rc::clone(subnode));
                        }
                    }
                    None => break,
                }
            }
        }
        return Some(Rc::clone(&root_node));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let root = build_tree(vec![
            Some(5),
            Some(3),
            Some(6),
            Some(2),
            Some(4),
            None,
            Some(7),
        ]);
        let key = 3;
        assert_eq!(
            delete_node(root, key),
            build_tree(vec![
                Some(5),
                Some(2),
                Some(6),
                None,
                Some(4),
                None,
                Some(7)
            ])
        );
    }
    #[test]
    fn run_tc2() {
        let root = build_tree(vec![
            Some(5),
            Some(3),
            Some(6),
            Some(2),
            Some(4),
            None,
            Some(7),
        ]);
        let key = 0;
        assert_eq!(
            delete_node(root, key),
            build_tree(vec![
                Some(5),
                Some(3),
                Some(6),
                Some(2),
                Some(4),
                None,
                Some(7)
            ])
        );
    }
}

fn main() {
    println!("Hello, world!");
}
