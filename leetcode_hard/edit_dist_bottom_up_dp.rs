const NOOP: i32 = 0;

fn min_distance(word1: String, word2: String) -> i32 {
    let str1 = word1.chars().collect::<Vec<char>>();
    let str2 = word2.chars().collect::<Vec<char>>();
    let n = str1.len();
    let m = str2.len();
    // Base cases
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for i in 0..=n {
        dp[i][0] = i as i32;
    }
    for j in 0..=m {
        dp[0][j] = j as i32;
    }

    //Recurrence
    for i in 1..=n {
        for j in 1..=m {
            match str1[i.checked_sub(1).unwrap()] == str2[j.checked_sub(1).unwrap()] {
                true => {
                    dp[i][j] = NOOP + dp[i.checked_sub(1).unwrap()][j.checked_sub(1).unwrap()];
                }
                false => {
                    dp[i][j] = i32::min(
                        1 + dp[i.checked_sub(1).unwrap()][j.checked_sub(1).unwrap()],
                        i32::min(
                            1 + dp[i.checked_sub(1).unwrap()][j],
                            1 + dp[i][j.checked_sub(1).unwrap()],
                        ),
                    );
                }
            }
        }
    }
    dp[n][m]
}

#[cfg(test)]
pub mod tests {
    use crate::min_distance;
    #[test]
    fn run_tc1() {
        let str1 = String::from("horse");
        let str2 = String::from("ros");
        assert_eq!(min_distance(str1, str2), 3);
    }
    #[test]
    fn run_tc2() {
        let str1 = String::from("intention");
        let str2 = String::from("execution");
        assert_eq!(min_distance(str1, str2), 5);
    }
}

fn main() {
    let str1 = String::from("horse");
    let str2 = String::from("ros");
    assert_eq!(min_distance(str1, str2), 3);
}
