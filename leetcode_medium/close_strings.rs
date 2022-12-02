use std::collections::HashSet;

fn close_strings(word1: String, word2: String) -> bool {
    if word1.len() != word2.len() {
        return false;
    }
    let (mut freq1, mut freq2) = (vec![0; 26], vec![0; 26]);
    for (c1, c2) in word1.chars().zip(word2.chars()) {
        freq1[(c1 as u8 - 'a' as u8) as usize] += 1;
        freq2[(c2 as u8 - 'a' as u8) as usize] += 1;
    }
    let hs1 = word1.chars().into_iter().collect::<HashSet<char>>();
    let hs2 = word2.chars().into_iter().collect::<HashSet<char>>();
    if hs1 != hs2 {
        return false;
    }
    freq1.sort_by(|a, b| b.cmp(a));
    freq2.sort_by(|a, b| b.cmp(a));

    freq1 == freq2
}

#[cfg(test)]
pub mod tests {
    use crate::close_strings;
    #[test]
    fn run_tc1() {
        let word1 = String::from("abc");
        let word2 = String::from("bca");
        assert_eq!(close_strings(word1, word2), true);
    }
    #[test]
    fn run_tc2() {
        let word1 = String::from("a");
        let word2 = String::from("aa");
        assert_eq!(close_strings(word1, word2), false);
    }
    #[test]
    fn run_tc3() {
        let word1 = String::from("cabbba");
        let word2 = String::from("abbccc");
        assert_eq!(close_strings(word1, word2), true);
    }
}

fn main() {
    println!("Hello, world!");
    let word1 = String::from("abc");
    let word2 = String::from("bca");
    assert_eq!(close_strings(word1, word2), true);
}
