use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::convert::Into;
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

impl Into<Rc<RefCell<TreeNode>>> for TreeNode {
    fn into(self) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(self))
    }
}

fn build_tree(buffer: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if buffer.len() == 0 {
        return None;
    }
    let root_rc: Rc<RefCell<TreeNode>> = TreeNode::new(buffer[0].unwrap()).into();
    let mut q = VecDeque::new();
    q.push_back(Rc::clone(&root_rc));
    let mut curr = Rc::clone(&root_rc);
    let mut left = true;

    for ele in buffer.iter().skip(1) {
        let new_ele: Option<Rc<RefCell<TreeNode>>>;
        match ele {
            Some(val) => {
                new_ele = Some(TreeNode::new(*val).into());
                q.push_back(Rc::clone(new_ele.as_ref().unwrap()));
            }
            None => {
                new_ele = None;
            }
        }
        if left {
            curr = q.pop_front().unwrap();
            curr.borrow_mut().left = new_ele;
            left = false;
        } else {
            curr.borrow_mut().right = new_ele;
            left = true;
        }
    }
    Some(root_rc)
}

// **************************************************************************
//                      Solution                                            //
// **************************************************************************

fn dfs(
    root: &Option<Rc<RefCell<TreeNode>>>,
    pattern: &mut HashMap<String, i32>,
    res: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
) -> String {
    if let Some(node) = root {
        let _node = node.borrow();

        let left_part = match _node.left {
            Some(ref subnode) => dfs(&Some(Rc::clone(subnode)), pattern, res),
            None => String::from("#"),
        };
        let right_part = match _node.right {
            Some(ref subnode) => dfs(&Some(Rc::clone(subnode)), pattern, res),
            None => String::from("#"),
        };
        let tok = vec![format!("{}", _node.val), left_part, right_part];
        let p = tok.join(",");
        match pattern.get(&p) {
            Some(val) if *val == 1 => {
                res.push(Some(Rc::clone(node)));
            }
            Some(_) => {}
            None => {}
        };
        pattern
            .entry(p.clone())
            .and_modify(|v| *v += 1)
            .or_insert(1);
        return p;
    }
    String::from("#")
}

fn find_duplicate_subtrees(
    root: Option<Rc<RefCell<TreeNode>>>,
) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    let mut res = vec![];
    let mut pattern = HashMap::new();
    dfs(&root, &mut pattern, &mut res);
    res
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let buffer = vec![
            Some(1),
            Some(2),
            Some(3),
            Some(4),
            None,
            Some(2),
            Some(4),
            None,
            None,
            Some(4),
        ];
        let root = build_tree(buffer);
        let ans = find_duplicate_subtrees(root);
        assert_eq!(
            ans,
            vec![
                build_tree(vec![Some(4)]),
                build_tree(vec![Some(2), Some(4)])
            ]
        );
    }
    #[test]
    fn run_tc2() {
        let buffer = vec![Some(2), Some(1), Some(1)];
        let root = build_tree(buffer);
        let ans = find_duplicate_subtrees(root);
        assert_eq!(ans, vec![build_tree(vec![Some(1)])]);
    }
    #[test]
    fn run_tc3() {
        let buffer = vec![Some(2), Some(2), Some(2), Some(3), None, Some(3), None];
        let root = build_tree(buffer);
        let ans = find_duplicate_subtrees(root);
        assert_eq!(
            ans,
            vec![
                build_tree(vec![Some(3)]),
                build_tree(vec![Some(2), Some(3)])
            ]
        );
    }
}

fn main() {
    let buffer = vec![
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        None,
        Some(2),
        Some(4),
        None,
        None,
        Some(4),
    ];
    let root = build_tree(buffer);
    let ans = find_duplicate_subtrees(root);
    assert_eq!(
        ans,
        vec![
            build_tree(vec![Some(4)]),
            build_tree(vec![Some(2), Some(4)])
        ]
    );
}
