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

fn solve(root: &Option<Rc<RefCell<TreeNode>>>) {
    if let Some(root_node) = root {
        let mut _root_node = root_node.borrow_mut();

        let left_val = match _root_node.left {
            Some(ref subnode) => subnode.borrow().val,
            None => 0,
        };

        let right_val = match _root_node.right {
            Some(ref subnode) => subnode.borrow().val,
            None => 0,
        };

        let child = left_val + right_val;

        if child >= _root_node.val {
            _root_node.val = child;
            return;
        } else {
            if _root_node.left.is_some() {
                match _root_node.left {
                    Some(ref subnode) => {
                        subnode.borrow_mut().val = _root_node.val;
                    }
                    None => {}
                };
            } else if _root_node.right.is_some() {
                match _root_node.right {
                    Some(ref subnode) => {
                        subnode.borrow_mut().val = _root_node.val;
                    }
                    None => {}
                }
            }
        }

        match _root_node.left {
            Some(ref subnode) => {
                solve(&Some(Rc::clone(subnode)));
            }
            None => {}
        };

        match _root_node.right {
            Some(ref subnode) => {
                solve(&Some(Rc::clone(subnode)));
            }
            None => {}
        };

        let left_val = match _root_node.left {
            Some(ref subnode) => subnode.borrow().val,
            None => 0,
        };

        let right_val = match _root_node.right {
            Some(ref subnode) => subnode.borrow().val,
            None => 0,
        };

        if _root_node.left.is_some() || _root_node.right.is_some() {
            _root_node.val = left_val + right_val;
        }
    }
}

fn children_sum_property(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(ref root_node) = root {
        solve(&Some(Rc::clone(root_node)));
        return Some(Rc::clone(root_node));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let root = build_tree(vec![
            Some(50),
            Some(10),
            Some(5),
            Some(2),
            Some(5),
            Some(1),
            Some(2),
        ]);
        assert_eq!(
            children_sum_property(root),
            build_tree(vec![
                Some(62),
                Some(55),
                Some(7),
                Some(50),
                Some(5),
                Some(5),
                Some(2),
            ])
        );
    }
}

fn main() {
    println!("Hello world");
}
