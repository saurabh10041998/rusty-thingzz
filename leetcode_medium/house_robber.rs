fn rob(nums: Vec<i32>) -> i32 {
    match nums.len() {
        0 => return 0,
        1 => return nums[0],
        2 => return i32::max(nums[0], nums[1]),
        _ => {}
    };
    let mut dp = vec![];
    dp.resize(nums.len(), 0);
    dp[0] = nums[0];
    dp[1] = i32::max(dp[0], nums[1]);
    for i in 2..nums.len() {
        dp[i] = i32::max(dp[i - 1], nums[i] + dp[i - 2]);
    }
    dp[nums.len() - 1]
}

#[cfg(test)]
pub mod tests {
    use crate::rob;
    #[test]
    fn run_tc1() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(rob(nums), 4);
    }

    #[test]
    fn run_tc2() {
        let nums = vec![2, 7, 9, 3, 1];
        assert_eq!(rob(nums), 12);
    }
}

fn main() {
    let nums = vec![1, 2, 3, 1];
    assert_eq!(rob(nums), 4);
}
