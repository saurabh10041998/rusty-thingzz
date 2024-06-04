use std::collections::HashMap;

fn longest_palindrome(s: String) -> i32 {
    let mut hs = HashMap::new();
    for c in s.chars() {
        hs.entry(c).and_modify(|f: &mut i32| *f += 1).or_insert(1);
    }
    let mut ans = 0;
    let mut hasodd = false;
    for (_k, v) in hs {
        if v & 1 == 0 {
            ans += v;
        } else {
            ans += v - 1;
            hasodd = true;
        }
    }
    match hasodd {
        true => ans + 1,
        false => ans,
    }
}

#[cfg(test)]
pub mod tests {
    use crate::longest_palindrome;
    #[test]
    fn run_tc1() {
        let s = String::from("abccccdd");
        assert_eq!(longest_palindrome(s), 7);
    }
    #[test]
    fn run_tc2() {
        let s = String::from("a");
        assert_eq!(longest_palindrome(s), 1);
    }
}

fn main() {
    let s = String::from("abccccdd");
    assert_eq!(longest_palindrome(s), 7);
}
