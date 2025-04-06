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

fn solve(
    root: &Option<Rc<RefCell<TreeNode>>>,
    p: &Option<Rc<RefCell<TreeNode>>>,
    q: &Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if let (Some(root_node), Some(p_node), Some(q_node)) = (root, p, q) {
        if root_node == p_node || root_node == q_node {
            return Some(Rc::clone(root_node));
        }

        let _root_node = root_node.borrow();
        let ans1 = match _root_node.left {
            Some(ref subnode) => solve(
                &Some(Rc::clone(subnode)),
                &Some(Rc::clone(p_node)),
                &Some(Rc::clone(q_node)),
            ),
            None => None,
        };
        let ans2 = match _root_node.right {
            Some(ref subnode) => solve(
                &Some(Rc::clone(subnode)),
                &Some(Rc::clone(p_node)),
                &Some(Rc::clone(q_node)),
            ),
            None => None,
        };

        if ans1.is_none() {
            return ans2;
        } else if ans2.is_none() {
            return ans1;
        } else {
            // both have is_some, so current  node is ans
            return Some(Rc::clone(root_node));
        }
    }
    None
}

fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(ref root_node) = root {
        return solve(&Some(Rc::clone(root_node)), &p, &q);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let root = build_tree(vec![
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
        ]);
        let p = find_node(&root, 5);
        let q = find_node(&root, 1);

        let ans = find_node(&root, 3);

        assert_eq!(lowest_common_ancestor(root, p, q), ans);
    }
    #[test]
    fn run_tc2() {
        let root = build_tree(vec![
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
        ]);
        let p = find_node(&root, 5);
        let q = find_node(&root, 4);

        let ans = find_node(&root, 5);

        assert_eq!(lowest_common_ancestor(root, p, q), ans);
    }

    #[test]
    fn run_tc3() {
        let root = build_tree(vec![Some(1), Some(2)]);
        let p = find_node(&root, 1);
        let q = find_node(&root, 2);

        let ans = find_node(&root, 1);

        assert_eq!(lowest_common_ancestor(root, p, q), ans);
    }
}

fn main() {
    println!("Hello world");
}
