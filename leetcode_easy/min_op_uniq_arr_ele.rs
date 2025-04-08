use std::collections::HashSet;

fn ceil(a: i32, b: i32) -> i32 {
    (a / b) + (a % b != 0) as i32
}

fn minimum_operations(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut hs = HashSet::new();
    for i in (0..n).rev() {
        if hs.contains(&nums[i]) {
            return ceil((i + 1) as i32, 3);
        }
        hs.insert(nums[i]);
    }
    0
}

#[cfg(test)]
mod tests {
    use crate::minimum_operations;
    #[test]
    fn run_tc1() {
        let nums = vec![1, 2, 3, 4, 2, 3, 3, 5, 7];
        assert_eq!(minimum_operations(nums), 2);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![4, 5, 6, 4, 4];
        assert_eq!(minimum_operations(nums), 2);
    }
    #[test]
    fn run_tc3() {
        let nums = vec![6, 7, 8, 9];
        assert_eq!(minimum_operations(nums), 0);
    }
}

fn main() {
    let nums = vec![1, 2, 3, 4, 2, 3, 3, 5, 7];
    assert_eq!(minimum_operations(nums), 2);
}
