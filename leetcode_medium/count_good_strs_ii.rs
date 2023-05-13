const MOD: i64 = i64::pow(10, 9) + 7;
fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
    let max_len = (high + i32::max(zero, one) + 1) as usize;
    let mut dp = vec![-1i64; max_len];
    for i in (high as usize + 1)..max_len {
        dp[i] = 0;
    }
    for i in (0..=high as usize).rev() {
        if i >= low as usize {
            dp[i] = 1;
        } else {
            dp[i] = 0;
        }
        dp[i] = (dp[i]
            + dp[usize::min(max_len - 1, i + zero as usize)]        // append 0 zero time.. do not exceed max_len
            + dp[usize::min(max_len - 1, i + one as usize)])        // append 1 one time.. do not exceed max_len
            % MOD;
    }
    (dp[0] % MOD) as i32
}

#[cfg(test)]
pub mod tests {
    use crate::count_good_strings;
    #[test]
    fn run_tc1() {
        let low = 3;
        let high = 3;
        let zero = 1;
        let one = 1;
        assert_eq!(count_good_strings(low, high, zero, one), 8);
    }
    #[test]
    fn run_tc2() {
        let low = 2;
        let high = 3;
        let zero = 1;
        let one = 2;
        assert_eq!(count_good_strings(low, high, zero, one), 5);
    }
}

fn main() {
    let low = 2;
    let high = 3;
    let zero = 1;
    let one = 2;
    assert_eq!(count_good_strings(low, high, zero, one), 8);
}
