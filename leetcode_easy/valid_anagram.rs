//! Solution for this problem is simple
//! Prepare hashmap / counter of characters in both strings s and t
//! and check if the both hashmaps are equal
//! Thankfully Rust has `eq` and `partialEq` traits implemented for HashMap
//! so we just have to call eq() method, and that does the job of
//! comparing two hashmaps for equality
use std::collections::HashMap;
fn is_anagram(s: String, t: String) -> bool {
    s.chars()
        .fold(HashMap::new(), |mut accum, ch| {
            accum.entry(ch).and_modify(|f| *f += 1).or_insert(1);
            accum
        })
        .eq(&t.chars().fold(HashMap::new(), |mut accum, ch| {
            accum.entry(ch).and_modify(|f| *f += 1).or_insert(1);
            accum
        }))
}

#[cfg(test)]
pub mod tests {
    use crate::is_anagram;
    #[test]
    fn run_tc1() {
        let s = String::from("anagram");
        let t = String::from("nagaram");
        assert!(is_anagram(s, t));
    }
    #[test]
    fn run_tc2() {
        let s = String::from("rat");
        let t = String::from("car");
        assert!(!is_anagram(s, t));
    }
}

fn main() {
    let s = String::from("anagram");
    let t = String::from("nagaram");
    assert!(is_anagram(s, t));
}
