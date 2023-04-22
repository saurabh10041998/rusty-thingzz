fn lcs(s: &Vec<char>, t: &Vec<char>) -> i32 {
    let n = s.len();
    let mut dp = vec![vec![0; n + 1]; n + 1];
    for i in 0..=n {
        dp[i][0] = 0;
        dp[0][i] = 0;
    }
    for i in 1..=n {
        for j in 1..=n {
            if s[i - 1] == t[j - 1] {
                dp[i][j] = 1 + dp[i - 1][j - 1];
            } else {
                dp[i][j] = i32::max(dp[i - 1][j], dp[i][j - 1]);
            }
        }
    }
    dp[n][n]
}


fn min_insertions(s: String) -> i32 {
    let n = s.len();
    let t = s.chars().rev().map(|c| c).collect::<Vec<char>>();
    let s = s.chars().collect::<Vec<char>>();
    n as i32 - lcs(&s, &t)
}

#[cfg(test)]
pub mod tests {
    use crate::min_insertions;
    #[test]
    fn run_tc1() {
        let s = String::from("zzazz");
        assert_eq!(min_insertions(s), 0);
    }
    #[test]
    fn run_tc2() {
        let s = String::from("mbadm");
        assert_eq!(min_insertions(s), 2);
    }
    #[test]
    fn run_tc3() {
        let s = String::from("leetcode");
        assert_eq!(min_insertions(s), 5);
    }
}

fn main() {
    let s = String::from("leetcode");
    assert_eq!(min_insertions(s), 5);
}
