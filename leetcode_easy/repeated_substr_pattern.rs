fn repeated_substring_pattern(s: String) -> bool {
    let n = s.len();
    for i in 1..=n / 2 {
        if n % i == 0 {
            let substring = &s[0..i];
            let mut repeated = String::new();
            for _ in 0..(n / i) {
                repeated.push_str(substring);
            }
            if repeated == s {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
pub mod tests {
    use crate::repeated_substring_pattern;
    #[test]
    fn run_tc1() {
        let s = String::from("abab");
        assert_eq!(repeated_substring_pattern(s), true);
    }
    #[test]
    fn run_tc2() {
        let s = String::from("aba");
        assert_eq!(repeated_substring_pattern(s), false);
    }
    #[test]
    fn run_tc3() {
        let s = String::from("abcabcabcabc");
        assert_eq!(repeated_substring_pattern(s), true);
    }
}

fn main() {
    let s = String::from("abcabcabcabc");
    assert_eq!(repeated_substring_pattern(s), true);
}
