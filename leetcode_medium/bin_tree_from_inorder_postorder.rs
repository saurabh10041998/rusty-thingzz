use std::cell::RefCell;
use std::collections::HashMap;
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

fn build_tree_for_test(buffer: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if buffer.len() == 0 {
        return None;
    }
    let root_rc = Rc::new(RefCell::new(TreeNode::new(buffer[0].unwrap())));
    let mut curr = Rc::clone(&root_rc);
    let mut q = VecDeque::new();
    q.push_back(Rc::clone(&root_rc));
    let mut left = true;
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

fn dfs_helper(
    index_map: &HashMap<i32, usize>,
    l: i32,
    r: i32,
    postorder: &mut Vec<i32>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if l > r {
        return None;
    }
    let root_rc = Rc::new(RefCell::new(TreeNode::new(postorder.pop().unwrap())));
    let idx = match index_map.get(&root_rc.borrow().val) {
        Some(val) => *val,
        None => unreachable!(),
    };
    root_rc.borrow_mut().right = dfs_helper(index_map, (idx + 1) as i32, r, postorder);
    root_rc.borrow_mut().left = dfs_helper(index_map, l, (idx as i32) - 1, postorder);
    Some(root_rc)
}

// ********************* Solution ******************************
fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut index_map = HashMap::new();
    for (idx, &ele) in inorder.iter().enumerate() {
        index_map.entry(ele).or_insert(idx);
    }
    let mut postorder = postorder;
    dfs_helper(&index_map, 0, (inorder.len() as i32) - 1, &mut postorder)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let inorder = vec![9, 3, 15, 20, 7];
        let postorder = vec![9, 15, 7, 20, 3];
        assert_eq!(
            build_tree(inorder, postorder),
            build_tree_for_test(vec![
                Some(3),
                Some(9),
                Some(20),
                None,
                None,
                Some(15),
                Some(7)
            ])
        );
    }
    #[test]
    fn run_tc2() {
        let inorder = vec![-1];
        let postorder = vec![-1];
        assert_eq!(
            build_tree(inorder, postorder),
            build_tree_for_test(vec![Some(-1)])
        );
    }
}
fn main() {
    let inorder = vec![-1];
    let postorder = vec![-1];
    assert_eq!(
        build_tree(inorder, postorder),
        build_tree_for_test(vec![Some(-1)])
    );
}
