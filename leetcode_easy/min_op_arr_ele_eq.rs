use std::collections::BTreeSet;

fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    let hs = nums.into_iter().collect::<BTreeSet<i32>>();
    if *hs.first().unwrap() < k {
        -1
    } else if *hs.first().unwrap() == k {
        (hs.len() - 1) as i32
    } else {
        hs.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::min_operations;
    #[test]
    fn run_tc1() {
        let nums = vec![5, 2, 5, 4, 5];
        let k = 2;
        assert_eq!(min_operations(nums, k), 2);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![2, 1, 2];
        let k = 2;
        assert_eq!(min_operations(nums, k), -1);
    }
    #[test]
    fn run_tc3() {
        let nums = vec![9, 7, 5, 3];
        let k = 1;
        assert_eq!(min_operations(nums, k), 4);
    }
}

fn main() {
    let nums = vec![5, 2, 5, 4, 5];
    let k = 2;
    assert_eq!(min_operations(nums, k), 2);
}
