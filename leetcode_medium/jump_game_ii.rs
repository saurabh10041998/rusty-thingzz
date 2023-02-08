fn helper(nums: &Vec<i32>, dp: &mut Vec<i32>, idx: usize) -> i32 {
    if idx >= nums.len() - 1 {
        return 0;
    }
    if dp[idx] != 100001 {
        return dp[idx];
    }
    for i in 1..=nums[idx] {
        let i = i as usize;
        dp[idx] = i32::min(dp[idx], 1 + helper(nums, dp, idx + i));
    }
    println!("{} {}", idx, dp[idx]);
    dp[idx]
}

fn jump(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut dp = vec![100001; n + 1];
    helper(&nums, &mut dp, 0)
}

#[cfg(test)]
pub mod tests {
    use crate::jump;
    #[test]
    fn run_tc1() {
        let nums = vec![2, 3, 1, 1, 4];
        assert_eq!(jump(nums), 2);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![2, 3, 0, 1, 4];
        assert_eq!(jump(nums), 2);
    }
}

fn main() {
    let nums = vec![2, 3, 0, 1, 4];
    assert_eq!(jump(nums), 2);
}
