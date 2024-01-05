fn length_of_lis(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut dp = vec![1; n];
    for i in 0..n {
        for j in (0..i).rev() {
            if nums[j] < nums[i] && dp[i] < dp[j] + 1 {
                dp[i] = dp[j] + 1;
            }
        }
    }
    dp.into_iter().max().unwrap()
}

#[cfg(test)]
pub mod tests {
    use crate::length_of_lis;
    #[test]
    fn run_tc1() {
        let nums = vec![10, 9, 2, 5, 3, 7, 101, 18];
        assert_eq!(length_of_lis(nums), 4);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![0, 1, 0, 3, 2, 3];
        assert_eq!(length_of_lis(nums), 4);
    }
    #[test]
    fn run_tc3() {
        let nums = vec![7, 7, 7, 7, 7, 7, 7];
        assert_eq!(length_of_lis(nums), 1);
    }
}

fn main() {
    let nums = vec![7, 7, 7, 7, 7, 7, 7];
    assert_eq!(length_of_lis(nums), 1);
}
