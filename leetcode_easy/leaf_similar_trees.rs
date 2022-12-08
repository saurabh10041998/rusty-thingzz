use std::rc::Rc;
use std::cell::RefCell;

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
            right: None 
        }
    }
}

fn get_leaves(root: &Option<Rc<RefCell<TreeNode>>>, ans: &mut Vec<i32>) {
    if let Some(node) = root {
        let node = node.borrow();

        if node.left.is_none() && node.right.is_none() {
            ans.push(node.val);
        }
        get_leaves(&node.left, ans);
        get_leaves(&node.right, ans);
    }
}

fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut l1 = vec![];
    let mut l2 = vec![];

    get_leaves(&root1, &mut l1);
    get_leaves(&root2, &mut l2);

    l1 == l2
}

// TODO : learn insert and create tree using these smart pointers

fn main() {
    println!("Hello, world!");
}
