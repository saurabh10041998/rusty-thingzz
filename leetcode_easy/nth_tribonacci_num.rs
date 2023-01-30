fn tribonacci(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        2 => 1,
        v => {
            let mut dp = vec![0; (v + 1) as usize];
            dp[0] = 0;
            dp[1] = 1;
            dp[2] = 1;
            for i in 3..=v {
                let idx = i as usize;
                dp[idx] = dp[idx - 1] + dp[idx - 2] + dp[idx - 3];
            }
            dp[v as usize]
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::tribonacci;
    #[test]
    fn run_tc1() {
        let n = 4;
        assert_eq!(tribonacci(n), 4);
    }
    #[test]
    fn run_tc2() {
        let n = 25;
        assert_eq!(tribonacci(n), 1389537);
    }
}
fn main() {
    let n = 4;
    assert_eq!(tribonacci(n), 4);
}
