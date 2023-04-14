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
        Some(ans) => return *ans,
        None => {}
    };
    if s[n - 1] == t[m - 1] {
        let ans = 1 + lcs(s, t, n - 1, m - 1, dp);
        dp.entry((n, m)).or_insert(ans);
        return ans;
    } else {
        let ans = i32::max(lcs(s, t, n - 1, m, dp), lcs(s, t, n, m - 1, dp));
        dp.entry((n, m)).or_insert(ans);
        return ans;
    }
}

fn longest_palindrome_string(s: String) -> i32 {
    let s = s.chars().collect::<Vec<char>>();
    let t = s.iter().rev().map(|c| *c).collect::<Vec<char>>();
    let mut dp = HashMap::new();
    lcs(&s, &t, s.len(), t.len(), &mut dp)
}

#[cfg(test)]
pub mod tests {
    use crate::longest_palindrome_string;
    #[test]
    fn run_tc1() {
        let s = String::from("bbbab");
        assert_eq!(longest_palindrome_string(s), 4);
    }
    #[test]
    fn run_tc2() {
        let s = String::from("cbbd");
        assert_eq!(longest_palindrome_string(s), 2);
    }
}

fn main() {
    let s = String::from("cbbd");
    assert_eq!(longest_palindrome_string(s), 2);
}
