const NOOP: i32 = 0;

fn min_distance(word1: String, word2: String) -> i32 {
    let str1 = word1.chars().collect::<Vec<char>>();
    let str2 = word2.chars().collect::<Vec<char>>();
    let n = str1.len();
    let m = str2.len();
    let mut prev = vec![0i32; m + 1]; // dp[i - 1]
    let mut curr = vec![0i32; m + 1]; // dp[i]

    for j in 0..=m {
        prev[j] = j as i32;
    }

    for i in 1..=n {
        curr[0] = i as i32;
        for j in 1..=m {
            match str1[i.checked_sub(1).unwrap()] == str2[j.checked_sub(1).unwrap()] {
                true => {
                    curr[j] = NOOP + prev[j.checked_sub(1).unwrap()];
                }
                false => {
                    curr[j] = i32::min(
                        1 + prev[j.checked_sub(1).unwrap()],
                        i32::min(1 + curr[j.checked_sub(1).unwrap()], 1 + prev[j]),
                    );
                }
            }
        }
        prev = curr.clone();
    }
    prev[m]
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
