use std::cell::RefCell;
use std::convert::Into;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    val: i32,
    is_leaf: bool,
    top_left: Option<Rc<RefCell<TreeNode>>>,
    top_right: Option<Rc<RefCell<TreeNode>>>,
    bottom_left: Option<Rc<RefCell<TreeNode>>>,
    bottom_right: Option<Rc<RefCell<TreeNode>>>,
}

impl Into<Rc<RefCell<TreeNode>>> for TreeNode {
    fn into(self) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(self))
    }
}

impl TreeNode {
    fn new(val: i32, is_leaf: bool) -> Self {
        TreeNode {
            val,
            is_leaf,
            top_left: None,
            top_right: None,
            bottom_left: None,
            bottom_right: None,
        }
    }
}

fn dfs(n: usize, r: usize, c: usize, grid: &Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut all_same = true;
    for i in 0..n {
        for j in 0..n {
            if grid[r][c] != grid[r + i][c + j] {
                all_same = false
            }
        }
    }
    let node = match all_same {
        true => Some(TreeNode::new(grid[r][c], true).into()),
        false => {
            let n = n / 2;
            let top_left = dfs(n, r, c, grid);
            let top_right = dfs(n, r, c + n, grid);
            let bottom_left = dfs(n, r + n, c, grid);
            let bottom_right = dfs(n, r + n, c + n, grid);
            let node: Rc<RefCell<TreeNode>> = TreeNode::new(1, false).into();
            node.borrow_mut().top_left = top_left;
            node.borrow_mut().top_right = top_right;
            node.borrow_mut().bottom_left = bottom_left;
            node.borrow_mut().bottom_right = bottom_right;
            Some(node)
        }
    };
    node
}

fn construct(grid: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    let n = grid.len();
    dfs(n, 0, 0, &grid)
}

// IMPORTANT + TODO : As of now, not sure how to write test for this code
// Just printing and mannually looking is the way :(
// To view the quad tree run following commands
// `cargo test -- --nocapture`
// Output will not be in good shape if you run all tests (Due to rust concurrency)
// To get output of individual test run this,
// `cargo test <test_name> -- --nocapture`
// For example, `cargo test run_tc2 -- --nocapture`
#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let grid = vec![vec![0, 1], vec![1, 0]];
        let root = construct(grid);
        dbg!(root);
        assert!(true);
    }
    #[test]
    fn run_tc2() {
        let grid = vec![
            vec![1, 1, 1, 1, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 0, 0, 0, 0],
            vec![1, 1, 1, 1, 0, 0, 0, 0],
        ];
        let root = construct(grid);
        dbg!(root);
        assert!(true);
    }
}

fn main() {
    let grid = vec![vec![0, 1], vec![1, 0]];
    let root = construct(grid);
    dbg!(root);
    assert!(true);
}
