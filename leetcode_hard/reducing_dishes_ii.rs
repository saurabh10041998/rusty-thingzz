fn max_satisfaction(satisfaction: Vec<i32>) -> i32 {
    let mut satisfaction = satisfaction;
    satisfaction.sort();
    let n = satisfaction.len();
    let mut dp = vec![vec![0; n + 1]; n + 1];
    for idx in (0..n).rev() {
        for time in (0..=idx).rev() {
            let include = satisfaction[idx] * (time as i32 + 1)  + dp[idx + 1][time + 1];
            let exclude = 0 + dp[idx + 1][time];
            dp[idx][time] = i32::max(include, exclude);
        }
    }
    dp[0][0]
}

#[cfg(test)]
pub mod tests {
    use crate::max_satisfaction;
    #[test]
    fn run_tc1() {
        let satisfaction = vec![-1, -8, 0, 5, -9];
        assert_eq!(max_satisfaction(satisfaction), 14);
    }
    #[test]
    fn run_tc2() {
        let satisfaction = vec![4, 3, 2];
        assert_eq!(max_satisfaction(satisfaction), 20);
    }
    #[test]
    fn run_tc3() {
        let satisfaction = vec![-1, -4, -5];
        assert_eq!(max_satisfaction(satisfaction), 0);
    }
}

fn main() {
    let satisfaction = vec![-1, -8, 0, 5, -9];
    assert_eq!(max_satisfaction(satisfaction), 14);
}
