use std::mem;
fn lcs(s: &Vec<char>, t: &Vec<char>) -> i32 {
    let n = s.len();
    let mut prev = vec![0; n + 1];
    let mut curr = vec![0; n + 1];
    for i in 0..=n {
        prev[i] = 0;
    }
    for i in 1..=n {
        curr[0] = 0;
        for j in 1..=n {
            if s[i - 1] == t[j - 1] {
                curr[j] = 1 + prev[j - 1];
            } else {
                curr[j] = i32::max(prev[j], curr[j - 1]);
            }
        }
        prev = mem::replace(&mut curr, vec![0; n + 1]);
    }
    prev[n]
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
