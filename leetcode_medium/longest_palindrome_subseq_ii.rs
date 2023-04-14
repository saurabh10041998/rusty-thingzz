fn lcs(s: &Vec<char>, t: &Vec<char>) -> i32 {
    let n = s.len();
    let m = t.len();
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for i in 0..=n {
        dp[i][0] = 0;
    }
    for j in 0..=m {
        dp[0][j] = 0;
    }
    for i in 1..=n {
        for j in 1..=m {
            if s[i - 1] == t[j - 1] {
                dp[i][j] = 1 + dp[i - 1][j - 1];
            } else {
                dp[i][j] = i32::max(dp[i - 1][j], dp[i][j - 1]);
            }
        }
    }
    dp[n][m]
}

fn longest_palindromic_subseq(s: String) -> i32 {
    let s = s.chars().collect::<Vec<char>>();
    let t = s.iter().rev().map(|c| *c).collect::<Vec<char>>();
    lcs(&s, &t)
}

#[cfg(test)]
pub mod tests {
    use crate::longest_palindromic_subseq;
    #[test]
    fn run_tc1() {
        let s = String::from("bbbab");
        assert_eq!(longest_palindromic_subseq(s), 4);
    }
    #[test]
    fn run_tc2() {
        let s = String::from("cbbd");
        assert_eq!(longest_palindromic_subseq(s), 2);
    }
}

fn main() {
    let s = String::from("cbbd");
    assert_eq!(longest_palindromic_subseq(s), 2);
}
