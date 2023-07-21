fn find_number_of_lis(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut dp = vec![1; n];
    let mut count = vec![1; n];
    let mut max_len = 1;
    for i in 0..n {
        for j in 0..i {
            if nums[j] < nums[i] {
                if dp[j] + 1 > dp[i] {
                    dp[i] = dp[j] + 1;
                    count[i] = count[j];
                } else if dp[j] + 1 == dp[i] {
                    count[i] += count[j];
                }
            }
        }
        max_len = i32::max(max_len, dp[i]);
    }
    let mut ans = 0;
    for (x, y) in dp.into_iter().zip(count.into_iter()) {
        if x == max_len {
            ans += y;
        }
    }
    ans
}

#[cfg(test)]
pub mod tests {
    use crate::find_number_of_lis;
    #[test]
    fn run_tc1() {
        let nums = vec![1, 3, 5, 4, 7];
        assert_eq!(find_number_of_lis(nums), 2);
    }
    #[test]
    fn run_tc2() {
        let nums = vec![2, 2, 2, 2, 2];
        assert_eq!(find_number_of_lis(nums), 5);
    }
}

fn main() {
    let nums = vec![2, 2, 2, 2, 2];
    assert_eq!(find_number_of_lis(nums), 5);
}
