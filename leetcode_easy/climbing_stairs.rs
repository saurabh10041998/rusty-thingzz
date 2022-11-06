fn _solve(n: i32, dp: &mut Vec<i32>) -> i32{
    if n <= 1 {
        dp[n as usize] = 1;
        return dp[n as usize];
    }
    if dp[n as usize ] != -1 {
        return dp[n as usize];
    }
    dp[n as usize] = _solve(n - 1, dp) + _solve(n - 2, dp);
    dp[n as usize]
}
fn climb_stairs(n: i32) -> i32 {
    let mut dp = vec![-1i32; (n + 1) as usize];
    _solve(n, &mut dp);
    dp[n as usize]
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn run_tc1() {
        let n = 2;
        assert_eq!(2, climb_stairs(n));
    }

    #[test]
    fn run_tc2() {
        let n = 3;
        assert_eq!(3, climb_stairs(n));
    }
}
fn main() {
    println!("Hello, world!");
    //climb_stairs(4);
    let n = 3;
    assert_eq!(3, climb_stairs(n));
}
