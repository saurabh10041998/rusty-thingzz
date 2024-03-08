use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl From<i32> for TreeNode {
    fn from(value: i32) -> Self {
        TreeNode {
            val: value,
            left: None,
            right: None,
        }
    }
}

fn build_tree(buffer: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if buffer.is_empty() {
        return None;
    }
    let root_rc: Rc<RefCell<TreeNode>> = Rc::new(RefCell::new(buffer[0].unwrap().into()));
    let mut curr = Rc::clone(&root_rc);
    let mut left = true;
    let mut q = VecDeque::new();
    q.push_back(Rc::clone(&root_rc));
    for ele in buffer.into_iter().skip(1) {
        let new_node: Option<Rc<RefCell<TreeNode>>>;
        match ele {
            Some(val) => {
                new_node = Some(Rc::new(RefCell::new(val.into())));
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
    Some(root_rc)
}

//===================================================
// Solution starts here
//===================================================

pub enum Distance {
    Upper(i32),
    Lower(i32),
}

fn __amount_of_time(
    root: Option<&Rc<RefCell<TreeNode>>>,
    start: i32,
    max_distance: &mut i32,
) -> Distance {
    if let Some(node) = root {
        let _node = node.borrow();

        let left_distance = match _node.left {
            Some(ref subnode) => __amount_of_time(Some(&Rc::clone(subnode)), start, max_distance),
            None => Distance::Lower(0),
        };
        let right_distance = match _node.right {
            Some(ref subnode) => __amount_of_time(Some(&Rc::clone(subnode)), start, max_distance),
            None => Distance::Lower(0),
        };

        if _node.val == start {
            match (left_distance, right_distance) {
                (Distance::Lower(left), Distance::Lower(right)) => {
                    *max_distance = i32::max(left, right);
                    return Distance::Upper(1);
                }
                (_, _) => unreachable!("Shouldn't happen"),
            }
        };
        match (left_distance, right_distance) {
            (Distance::Lower(left), Distance::Lower(right)) => {
                return Distance::Lower(i32::max(left, right) + 1);
            }
            (Distance::Lower(left), Distance::Upper(right)) => {
                let distance = left + right;
                *max_distance = i32::max(*max_distance, distance);
                return Distance::Upper(right + 1);
            }
            (Distance::Upper(left), Distance::Lower(right)) => {
                let distance = left + right;
                *max_distance = i32::max(*max_distance, distance);
                return Distance::Upper(left + 1);
            }
            (Distance::Upper(_), Distance::Upper(_)) => {
                unreachable!(
                    "Shouldn't happen unless there more than occurance of {}",
                    start
                );
            }
        }
    }
    Distance::Lower(0)
}

fn amount_of_time(root: Option<Rc<RefCell<TreeNode>>>, start: i32) -> i32 {
    let mut max_distance = 0;
    let _ = __amount_of_time(root.as_ref(), start, &mut max_distance);
    max_distance
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let buffer = vec![
            Some(1),
            Some(5),
            Some(3),
            None,
            Some(4),
            Some(10),
            Some(6),
            Some(9),
            Some(2),
        ];
        let root = build_tree(buffer);
        let start = 3;
        assert_eq!(amount_of_time(root, start), 4);
    }
    #[test]
    fn run_tc2() {
        let buffer = vec![Some(1)];
        let root = build_tree(buffer);
        let start = 1;
        assert_eq!(amount_of_time(root, start), 0);
    }
}

fn main() {
    let buffer = vec![
        Some(1),
        Some(5),
        Some(3),
        None,
        Some(4),
        Some(10),
        Some(6),
        Some(9),
        Some(2),
    ];
    let root = build_tree(buffer);
    let start = 3;
    assert_eq!(amount_of_time(root, start), 4);
}
