use std::collections::HashMap;

fn is_anagram(s: String, t: String) -> bool {
    let (mut hm1, mut hm2) = (HashMap::new(), HashMap::new());
    for c1 in s.chars() {
        hm1.entry(c1).and_modify(|v| *v += 1).or_insert(1);
    }
    for c2 in t.chars() {
        hm2.entry(c2).and_modify(|v| *v += 1).or_insert(1);
    }
    hm1 == hm2
}
#[cfg(test)]
pub mod tests {
    use crate::is_anagram;
    #[test]
    fn run_tc1() {
        let s = String::from("anagram");
        let t = String::from("nagaram");
        assert_eq!(is_anagram(s, t), true);
    }
    #[test]
    fn run_tc2() {
        let s = String::from("rat");
        let t = String::from("car");
        assert_eq!(is_anagram(s, t), false);
    }
}
fn main() {
    let s = String::from("rat");
    let t = String::from("car");
    assert_eq!(is_anagram(s, t), false);
}
