use std::collections::HashMap;

fn lcs(
    s: &Vec<char>,
    t: &Vec<char>,
    n: usize,
    m: usize,
    dp: &mut HashMap<(usize, usize), i32>,
) -> i32 {
    if n == 0 || m == 0 {
        return 0;
    }
    match dp.get(&(n, m)) {
        Some(val) => return *val,
        None => {}
    }
    if s[n - 1] == t[m - 1] {
        let ans = 1 + lcs(s, t, n - 1, m - 1, dp);
        dp.entry((n, m)).or_insert(ans);
        return ans;
    }
    let res = i32::max(lcs(s, t, n - 1, m, dp), lcs(s, t, n, m - 1, dp));
    dp.entry((n, m)).or_insert(res);
    res
}

fn min_insertions(s: String) -> i32 {
    let mut dp = HashMap::new();
    let n = s.len();
    let t = s.chars().rev().map(|c| c).collect::<Vec<char>>();
    let s = s.chars().collect::<Vec<char>>();
    n as i32 - lcs(&s, &t, n, n, &mut dp)
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
