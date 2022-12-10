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
    let root_rc = Rc::new(RefCell::new(TreeNode::new(buffer[0].unwrap())));
    let mut q = VecDeque::new();
    q.push_back(Rc::clone(&root_rc));
    let mut is_left = true;
    let mut cur = Rc::clone(&root_rc);
    for num in buffer.iter().skip(1) {
        let new_node;
        match num {
            Some(v) => {
                new_node = Some(Rc::new(RefCell::new(TreeNode::new(*v))));
                q.push_back(Rc::clone(new_node.as_ref().unwrap()));
            }
            None => {
                new_node = None;
            }
        }
        if is_left {
            // q will always be full till we process last node
            cur = q.pop_front().unwrap();
            cur.borrow_mut().left = new_node;
            is_left = false;
        } else {
            cur.borrow_mut().right = new_node;
            is_left = true;
        }
    }
    Some(Rc::clone(&root_rc))
}

// Solution.
fn get_total_sum(root: &Option<Rc<RefCell<TreeNode>>>) -> i64 {
    if let Some(node) = root {
        let _n = node.borrow();
        let leftsum = match _n.left {
            Some(ref subnode) => get_total_sum(&Some(Rc::clone(subnode))),
            None => 0,
        };
        let righsum = match _n.right {
            Some(ref subnode) => get_total_sum(&Some(Rc::clone(subnode))),
            None => 0,
        };
        return leftsum + righsum + _n.val as i64;
    }
    0
}

fn get_sum_under(root: &Option<Rc<RefCell<TreeNode>>>, total_sum: i64, result: &mut i64) -> i64 {
    if let Some(node) = root {
        let _n = node.borrow();
        let leftsum = match _n.left {
            Some(ref subnode) => get_sum_under(&Some(Rc::clone(subnode)), total_sum, result),
            None => 0,
        };
        let rightsum = match _n.right {
            Some(ref subnode) => get_sum_under(&Some(Rc::clone(subnode)), total_sum, result),
            None => 0,
        };
        let sum = leftsum + rightsum + _n.val as i64;
        *result = i64::max(*result, (total_sum - sum) * sum);
        return sum;
    }
    0
}

fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let total_sum = get_total_sum(&root);
    let mut result = 0;
    get_sum_under(&root, total_sum, &mut result);
    let modulo = i64::pow(10, 9) + 7;
    (result % modulo) as i32
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let buffer = vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)];
        let root = build_tree(buffer);
        assert_eq!(max_product(root), 110);
    }

    #[test]
    fn run_tc2() {
        let buffer = vec![
            Some(1),
            None,
            Some(2),
            Some(3),
            Some(4),
            None,
            None,
            Some(5),
            Some(6),
        ];
        let root = build_tree(buffer);
        assert_eq!(max_product(root), 90);
    }
}

fn main() {
    let buffer = vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)];
    let root = build_tree(buffer);
    dbg!(root);
    let buffer = vec![
        Some(1),
        None,
        Some(2),
        Some(3),
        Some(4),
        None,
        None,
        Some(5),
        Some(6),
    ];
    let root = build_tree(buffer);
    dbg!(root);
    let buffer = vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)];
    let root = build_tree(buffer);
    assert_eq!(max_product(root), 110);
}
