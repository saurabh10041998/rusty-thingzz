fn num_squares(n: i32) -> i32 {
    let mut dp = vec![];
    dp.resize((n + 1) as usize, n);
    dp[0] = 0;
    for i in 1..(n + 1) as usize {
        for j in 1..(n + 1) as usize {
            match i.checked_sub(j * j) {
                Some(_) => {
                    dp[i] = i32::min(dp[i], 1 + dp[i - (j * j)]);
                },
                None => { 
                    break;
                }
            }
        }
    }
    dp[n as usize]
}

#[cfg(test)]
pub mod tests {
    use crate::num_squares;
    #[test]
    fn run_tc1() {
        let n = 12;
        assert_eq!(num_squares(n), 3);
    }
    #[test]
    fn run_tc2() {
        let n = 13;
        assert_eq!(num_squares(n), 2);
    }
}
fn main() {
    let n = 13;
    assert_eq!(num_squares(n), 2);
}
