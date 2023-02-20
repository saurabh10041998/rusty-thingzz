fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    match nums.binary_search(&target) {
        Ok(idx) => idx as i32,
        Err(idx) => idx as i32,
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
