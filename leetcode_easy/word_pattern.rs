use std::collections::HashMap;

fn word_pattern(pattern: String, s: String) -> bool {
    let mut w_to_p = HashMap::new();
    let mut p_to_w: HashMap<char, String> = HashMap::new();
    let words = s.split_ascii_whitespace().collect::<Vec<&str>>();
    let tokens = pattern.chars().collect::<Vec<char>>();
    if words.len() != tokens.len() {
        return false;
    }
    for (&word, &pat) in words.iter().zip(tokens.iter()) {
        match w_to_p.get(&String::from(word)) {
            Some(v) => {
                if *v != pat {
                    return false;
                } else {
                }
            }
            None => {
                w_to_p.insert(String::from(word), pat);
            }
        };
        match p_to_w.get(&pat) {
            Some(v) => {
                if v.as_str() != word {
                    return false;
                } else {
                }
            }
            None => {
                p_to_w.insert(pat, String::from(word));
            }
        };
    }
    true
}
#[cfg(test)]
pub mod tests {
    use crate::word_pattern;
    #[test]
    fn run_tc1() {
        let pattern = String::from("abba");
        let s = String::from("dog cat cat dog");
        assert_eq!(word_pattern(pattern, s), true);
    }
    #[test]
    fn run_tc2() {
        let pattern = String::from("abba");
        let s = String::from("dog cat cat fish");
        assert_eq!(word_pattern(pattern, s), false);
    }
    #[test]
    fn run_tc3() {
        let pattern = String::from("aaaa");
        let s = String::from("dog cat cat dog");
        assert_eq!(word_pattern(pattern, s), false);
    }
    #[test]
    fn run_tc4() {
        let pattern = String::from("aaa");
        let s = String::from("aa aa aa aa");
        assert_eq!(word_pattern(pattern, s), false);
    }
}
fn main() {
    let pattern = String::from("abba");
    let s = String::from("dog cat cat dog");
    assert_eq!(word_pattern(pattern, s), true);
}
