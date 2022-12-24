fn num_tilings(n: i32) -> i32 { 
    let md = i32::pow(10, 9) + 7;
    let mut dp = vec![0; 1001];
    // Manually solved and calculated
    // number of ways for following n's
    dp[0] = 0;
    dp[1] = 1;
    dp[2] = 2;
    dp[3] = 5;
    
    // recursive eq : dp[n] = 2 * dp[n - 1] + dp[n - 3];
    for i in 4..(n + 1) as usize {
        dp[i] = ((2 * dp[i - 1]) % md + dp[i - 3]) % md;
    }
    dp[n as usize]
}

#[cfg(test)]
pub mod tests {
    use crate::num_tilings;
    #[test]
    fn run_tc1() {
        let n = 3;
        assert_eq!(num_tilings(n), 5);
    }
    #[test]
    fn run_tc2() {
        let n = 1;
        assert_eq!(num_tilings(n), 1);
    }
}

fn main() {
    num_tilings(3);
}
