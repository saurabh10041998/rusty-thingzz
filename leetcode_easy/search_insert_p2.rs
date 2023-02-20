use std::collections::BTreeSet;
use std::ops::Bound::*;

fn search_insert(nums:Vec<i32>, target: i32) -> i32 { 
    let mut nums = nums;
    let tree = nums.drain(..).collect::<BTreeSet<i32>>();
    let before = tree.range((Unbounded, Included(target)));
    let before = before.cloned().collect::<Vec<i32>>();
    match before.last() {
        Some(x) if *x == target => {
            (before.len() - 1)as i32
        },
        Some(_) => before.len() as i32,
        None => before.len() as i32
    }
}


#[cfg(test)]
pub mod tests {
    use crate::search_insert;
    #[test]
    fn run_tc1() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;
        assert_eq!(search_insert(nums, target), 2);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![1, 3, 5, 6];
        let target = 2;
        assert_eq!(search_insert(nums, target), 1);
    }
    #[test]
    fn run_tc3() {
        let nums = vec![1, 3, 5, 6];
        let target = 7;
        assert_eq!(search_insert(nums, target), 4);
    }
}

fn main() {
    let nums = vec![1, 3, 5, 6];
    let target = 5;
    assert_eq!(search_insert(nums, target), 2);
}